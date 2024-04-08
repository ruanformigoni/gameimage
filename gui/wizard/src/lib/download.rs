use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use downloader::Downloader;

use url as Url;

use anyhow::anyhow as ah;

use crate::log;
use crate::common;

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
  // If sha exists verify
  if path_file_dest.exists()
  {
    log!("File exists: '{:?}'", path_file_dest);
    f_finish();
    return Ok(());
  } // if

  // Try to remove files if failed
  log!("SHA check failed for '{:?}'", path_file_dest);
  let _ = std::fs::remove_file(path_file_dest.clone());

  // Get parent directory of file
  let dir_download = path_file_dest.parent().ok_or(ah!("Failed to acquire parent dir"))?;

  // Downloader instance
  let mut downloader = Downloader::builder()
    .download_folder(dir_download)
    .parallel_requests(1)
    .build()?;

  // Configure download
  let url = some_url.ok_or(ah!("Invalid url"))?.clone();
  #[cfg(not(feature = "tui"))] // Disable progress bar in terminal
  let dl_url = downloader::Download::new(url.as_str())
    .progress(SimpleReporter::create(f_begin.clone(), f_update.clone()))
    .file_name(&path_file_dest);

  // Fetch file
  log!("Start download file");
  for i in downloader.download(&[dl_url])?.pop().ok_or(ah!("Download failure"))??.status
  {
    if i.1 != 200
    {
      return Err(ah!("Connection error {}", i.1));
    }
  } // if

  // Set downloaded file as executable
  std::fs::set_permissions(path_file_dest.clone(), std::fs::Permissions::from_mode(0o766))?;

  // Finishing callback
  f_finish();

  Ok(())
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
