// pub fn resize() {{{
pub fn resize(path_out : std::path::PathBuf, path_in : std::path::PathBuf, width : u32, height : u32) -> anyhow::Result<()>
{
  let mut img = image::io::Reader::open(path_in)?.decode()?;
  img = img.resize(width, height, image::imageops::FilterType::CatmullRom);
  Ok(img.save(path_out)?)
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
