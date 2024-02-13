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

// Calculates the scaling factor from a 1920x1080 baseline
pub fn factor() -> Option<f32>
{
  // Get info
  let some_info = compute();

  if some_info.is_none()
  {
    return None;
  } // if

  let info = some_info.unwrap();

  // Calculate factor
  let width_base : u32 = 1920;
  let height_base : u32 = 1080;
  // Consider the width as the greatest value, (for flipped screens)
  let width_curr : u32 = if info.width > info.height { info.width } else { info.height };
  let height_curr : u32 = if info.width > info.height { info.height } else { info.width };
  // Create delta
  // // == 0 eq current dimension
  // // >  0 gt current dimension
  // // <  0 lt current dimension
  let width_delta : u32 = width_curr - width_base;
  let height_delta : u32 = height_curr - height_base;

  // Either width or height fit the baseline
  if width_delta == 0 || height_delta == 0
  {
    return Some(1.0);
  } // if

  // Positive deltas, should increase scale
  // or Negative deltas, should reduce scale
  let factor_width : f32 = width_curr as f32 / width_base as f32;
  let factor_height : f32 = height_curr as f32 / height_base as f32;

  Some(factor_width.min(factor_height))
}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
