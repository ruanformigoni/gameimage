use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::io::prelude::*;
use downloader::Downloader;

use url as Url;

use anyhow::anyhow as ah;

use crate::log;
use crate::common;
use crate::common::PathBufExt;

// struct SimpleReporterPrivate {{{
struct SimpleReporterPrivate
{
  last_update: std::time::Instant,
  max_progress: Option<u64>,
  message: String,
} // }}}

// struct SimpleReporter {{{
struct SimpleReporter
{
  private: std::sync::Mutex<Option<SimpleReporterPrivate>>,
  f_begin: std::sync::Mutex<Box<dyn FnMut() + Send + Sync + 'static>>,
  f_update: std::sync::Mutex<Box<dyn FnMut(f64) + Send + Sync + 'static>>,
} // }}}

// impl SimpleReporter {{{
impl SimpleReporter
{
  #[cfg(not(feature = "tui"))]
  fn create<F,G>(f_begin : F, f_update : G) -> std::sync::Arc<Self>
  where
    F: FnMut() + Send + Sync + 'static,
    G: FnMut(f64) + Send + Sync + 'static,
  {
    std::sync::Arc::new(Self
    {
      private: std::sync::Mutex::new(None),
      f_begin: std::sync::Mutex::new(Box::new(f_begin)),
      f_update: std::sync::Mutex::new(Box::new(f_update)),
    })
  }
} // }}}

// impl downloader::progress::Reporter for SimpleReporter {{{
impl downloader::progress::Reporter for SimpleReporter
{
  fn setup(&self, max_progress: Option<u64>, message: &str)
  {
    self.f_begin.lock().unwrap()();

    let private = SimpleReporterPrivate
    {
      last_update: std::time::Instant::now(),
      max_progress,
      message: message.to_owned(),
    };

    let mut guard = self.private.lock().unwrap();
    *guard = Some(private);
  }

  fn progress(&self, current: u64)
  {
    if let Some(p) = self.private.lock().unwrap().as_mut()
    {
      let max_bytes = match p.max_progress
      {
        Some(bytes) => bytes,
        None => 0,
      };

      if p.last_update.elapsed().as_millis() >= 1000
      {
        log!("Fetch {} of {} bytes. [{}]", current, max_bytes, p.message);
        p.last_update = std::time::Instant::now();
        let f64_progress = (current as f64 / max_bytes as f64) * 100.0 as f64;
        log!("Progress: {}%", f64_progress);
        self.f_update.lock().unwrap()(f64_progress);
      }

    }
  }

  fn set_message(&self, message: &str)
  {
    log!("test file: Message changed to: {}", message);
  }

  fn done(&self)
  {
    let mut guard = self.private.lock().unwrap();
    *guard = None;
    log!("test file: [DONE]");
  }
} // }}}

// sha() {{{
pub fn sha(file_sha : PathBuf, file_target: PathBuf) -> anyhow::Result<()>
{
  // Read sha
  let mut vec_sha : Vec<u8> = vec![0;64];
  let _ = std::fs::File::open(file_sha)?.read_exact(&mut vec_sha);
  let str_ref_sha = String::from_utf8(vec_sha)?;

  // Verify sha
  let str_target_sha = sha256::try_digest(&file_target)?;
  if str_target_sha != str_ref_sha
  {
    return Err(ah!("SHA verify failed, expected '{}', got '{}", str_ref_sha, str_target_sha));
  } // if

  log!("SHA verify successful for {}", file_target.string());

  Ok(())
} // }}}

// download {{{
pub fn download<F,G,H>(some_url : Option<Url::Url>
  , path_file_dest : PathBuf
  , f_begin : F
  , f_update : G
  , mut f_finish : H) -> anyhow::Result<()>
where
  F: FnMut() + Send + Sync + 'static + Clone,
  G: FnMut(f64) + Send + Sync + 'static + Clone,
  H: FnMut() + Send + Sync + 'static + Clone,
{
  // Get sha file name
  let path_file_dest_sha = path_file_dest.append_extension(".sha256sum");

  // If sha exists verify
  if sha(path_file_dest_sha.clone(), path_file_dest.clone()).is_ok()
  {
    log!("File exists, SHA check successful for '{:?}'", path_file_dest);
    f_finish();
    return Ok(());
  } // if

  // Try to remove files if failed
  log!("SHA check failed for '{:?}'", path_file_dest);
  let _ = std::fs::remove_file(path_file_dest.clone());
  let _ = std::fs::remove_file(path_file_dest_sha.clone());

  // Get parent directory of file
  let dir_download = path_file_dest.parent().ok_or(ah!("Failed to acquire parent dir"))?;

  // Downloader instance
  let mut downloader = Downloader::builder()
    .download_folder(dir_download)
    .parallel_requests(1)
    .build()?;

  // Configure download
  let url = some_url.ok_or(ah!("Invalid url"))?.clone();
  let url_sha = Url::Url::parse(&format!("{}.sha256sum", &url))?;
  #[cfg(not(feature = "tui"))] // Disable progress bar in terminal
  let dl_url = downloader::Download::new(url.as_str())
    .progress(SimpleReporter::create(f_begin.clone(), f_update.clone()))
    .file_name(&path_file_dest);
  #[cfg(not(feature = "tui"))] // Disable progress bar in terminal
  let dl_sha = downloader::Download::new(url_sha.as_str())
    .progress(SimpleReporter::create(f_begin, f_update))
    .file_name(&(path_file_dest_sha));

  // Fetch sha
  log!("Start download sha");
  let _ = downloader.download(&[dl_sha])?.pop().ok_or(ah!("Download failure"))??;

  // Fetch file
  log!("Start download file");
  for i in downloader.download(&[dl_url])?.pop().ok_or(ah!("Download failure"))??.status
  {
    if i.1 != 200
    {
      return Err(ah!("Connection error {}", i.1));
    }
  } // if

  // Re-create the SHA file with the same SHA & correct file name
  let mut sha256_content = std::fs::read_to_string(&path_file_dest_sha)?
    .split(' ')
    .next()
    .ok_or(ah!("Could not get SHA value from SHA file"))?
    .to_owned();
  sha256_content.push(' ');
  sha256_content.push_str(path_file_dest.string().as_str());
  std::fs::write(&path_file_dest_sha, sha256_content)?;

  // Set downloaded file as executable
  std::fs::set_permissions(path_file_dest.clone(), std::fs::Permissions::from_mode(0o766))?;

  // Check sha
  sha(path_file_dest_sha.clone(), path_file_dest.clone())?;

  // Finishing callback
  f_finish();

  Ok(())
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
