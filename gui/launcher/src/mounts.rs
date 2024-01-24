use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

pub fn mounts() -> anyhow::Result<Vec<(PathBuf, PathBuf, PathBuf, PathBuf)>>
{
  let vec_entries : Vec<DirEntry> = fs::read_dir("/fim/mount")?
    .filter_map(|e| { e.ok() })
    .filter(|e|{ e.path().is_dir() })
    .collect();

  let mut vec_pairs : Vec<(PathBuf, PathBuf, PathBuf, PathBuf)> = vec![];

  for entry in vec_entries
  {
    let path_entry = entry.path();
    let path_icon = path_entry.join("icon").join("icon.png");
    let path_icon_grayscale = path_entry.join("icon").join("icon.grayscale.png");
    let path_boot = path_entry.join("boot");
    if path_icon.exists() && path_boot.exists()
    {
      vec_pairs.push((path_boot, path_entry, path_icon, path_icon_grayscale));
    } // if
  } // for

  Ok(vec_pairs)
}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
