use display_info::DisplayInfo;

// Returns the display with the largest relationship resolution * scaling
pub fn compute() -> Option<DisplayInfo>
{
  if let Some(mut display_infos) = DisplayInfo::all().ok()
  {
    // Sort elements
    display_infos.sort_by(|a, b|
    {
      let prod_a = a.width * a.height * a.scale_factor.ceil() as u32 * 2;
      let prod_b = b.width * b.height * b.scale_factor.ceil() as u32 * 2;
      prod_a.cmp(&prod_b)
    });
    // Get largest element
    if let Some(display_info) = display_infos.last()
    {
      return Some(*display_info);
    }
  }
  None
}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
