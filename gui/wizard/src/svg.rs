#![allow(dead_code)]
#![allow(unused_variables)]

use crate::scaling;

const ICON_HOME: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-house-fill" viewBox="0 0 16 16">
  <path d="M8.707 1.5a1 1 0 0 0-1.414 0L.646 8.146a.5.5 0 0 0 .708.708L8 2.207l6.646 6.647a.5.5 0 0 0 .708-.708L13 5.793V2.5a.5.5 0 0 0-.5-.5h-1a.5.5 0 0 0-.5.5v1.293z"/>
  <path d="m8 3.293 6 6V13.5a1.5 1.5 0 0 1-1.5 1.5h-9A1.5 1.5 0 0 1 2 13.5V9.293z"/>
</svg>
"##;

const ICON_BACKGROUND: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" class="bi bi-play-fill" viewBox="0 0 30 20">
  <rect width="100%" height="100%" fill="#2A2E32" opacity="0.65"></rect>
</svg>
"##;

const ICON_CONFIGURE: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" viewBox="0 0 16 16">
  <path d="M1 0 0 1l2.2 3.081a1 1 0 0 0 .815.419h.07a1 1 0 0 1 .708.293l2.675 2.675-2.617 2.654A3.003 3.003 0 0 0 0 13a3 3 0 1 0 5.878-.851l2.654-2.617.968.968-.305.914a1 1 0 0 0 .242 1.023l3.27 3.27a.997.997 0 0 0 1.414 0l1.586-1.586a.997.997 0 0 0 0-1.414l-3.27-3.27a1 1 0 0 0-1.023-.242L10.5 9.5l-.96-.96 2.68-2.643A3.005 3.005 0 0 0 16 3c0-.269-.035-.53-.102-.777l-2.14 2.141L12 4l-.364-1.757L13.777.102a3 3 0 0 0-3.675 3.68L7.462 6.46 4.793 3.793a1 1 0 0 1-.293-.707v-.071a1 1 0 0 0-.419-.814zm9.646 10.646a.5.5 0 0 1 .708 0l2.914 2.915a.5.5 0 0 1-.707.707l-2.915-2.914a.5.5 0 0 1 0-.708M3 11l.471.242.529.026.287.445.445.287.026.529L5 13l-.242.471-.026.529-.445.287-.287.445-.529.026L3 15l-.471-.242L2 14.732l-.287-.445L1.268 14l-.026-.529L1 13l.242-.471.026-.529.445-.287.287-.445.529-.026z"/>
</svg>
"#;

const ICON_BACK: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" viewBox="0 0 16 16">
  <path fill-rule="evenodd" d="M15 8a.5.5 0 0 0-.5-.5H2.707l3.147-3.146a.5.5 0 1 0-.708-.708l-4 4a.5.5 0 0 0 0 .708l4 4a.5.5 0 0 0 .708-.708L2.707 8.5H14.5A.5.5 0 0 0 15 8"/>
</svg>
"#;

const ICON_PLAY: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" viewBox="0 0 16 16">
  <path d="m11.596 8.697-6.363 3.692c-.54.313-1.233-.066-1.233-.697V4.308c0-.63.692-1.01 1.233-.696l6.363 3.692a.802.802 0 0 1 0 1.393z"/>
</svg>
"#;

const ICON_LIST: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" viewBox="0 0 16 16">
  <path fill-rule="evenodd" d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5"/>
</svg>
"#;

const ICON_ADD: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-plus-lg" viewBox="0 0 16 16">
  <path fill-rule="evenodd" d="M8 2a.5.5 0 0 1 .5.5v5h5a.5.5 0 0 1 0 1h-5v5a.5.5 0 0 1-1 0v-5h-5a.5.5 0 0 1 0-1h5v-5A.5.5 0 0 1 8 2"/>
</svg>
"#;

const ICON_DEL: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-trash2-fill" viewBox="0 0 16 16">
  <path d="M2.037 3.225A.7.7 0 0 1 2 3c0-1.105 2.686-2 6-2s6 .895 6 2a.7.7 0 0 1-.037.225l-1.684 10.104A2 2 0 0 1 10.305 15H5.694a2 2 0 0 1-1.973-1.671zm9.89-.69C10.966 2.214 9.578 2 8 2c-1.58 0-2.968.215-3.926.534-.477.16-.795.327-.975.466.18.14.498.307.975.466C5.032 3.786 6.42 4 8 4s2.967-.215 3.926-.534c.477-.16.795-.327.975-.466-.18-.14-.498-.307-.975-.466z"/>
</svg>
"#;

const ICON_JOYSTICK: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-joystick" viewBox="0 0 16 16">
  <path d="M10 2a2 2 0 0 1-1.5 1.937v5.087c.863.083 1.5.377 1.5.726 0 .414-.895.75-2 .75s-2-.336-2-.75c0-.35.637-.643 1.5-.726V3.937A2 2 0 1 1 10 2"/>
  <path d="M0 9.665v1.717a1 1 0 0 0 .553.894l6.553 3.277a2 2 0 0 0 1.788 0l6.553-3.277a1 1 0 0 0 .553-.894V9.665c0-.1-.06-.19-.152-.23L9.5 6.715v.993l5.227 2.178a.125.125 0 0 1 .001.23l-5.94 2.546a2 2 0 0 1-1.576 0l-5.94-2.546a.125.125 0 0 1 .001-.23L6.5 7.708l-.013-.988L.152 9.435a.25.25 0 0 0-.152.23"/>
</svg>
"#;

macro_rules! icon
{
  ($func_name:ident, $icon:expr, $size_1:expr, $size_2:expr) =>
  {
    pub fn $func_name(scale : f32) -> String
    {
      let scaling = scaling::factor().unwrap_or(1.0);

      let size_1 = ($size_1 as f32 * scaling * scale) as i32;
      let str_size_1 = size_1.to_string();

      let size_2 = ($size_2 as f32 * scaling * scale) as i32;
      let str_size_2 = size_2.to_string();

      let mut result = $icon.replacen("{}", str_size_1.as_str(), 1);
      result = result.replacen("{}", str_size_2.as_str(), 1);
      result
    }
  }
}


icon!(icon_home, ICON_HOME, 16, 16);
icon!(icon_background, ICON_BACKGROUND, 317, 60);
icon!(icon_configure, ICON_CONFIGURE, 24, 24);
icon!(icon_back, ICON_BACK, 20, 20);
icon!(icon_play, ICON_PLAY, 28, 28);
icon!(icon_list, ICON_LIST, 16, 16);
icon!(icon_add, ICON_ADD, 24, 24);
icon!(icon_del, ICON_DEL, 16, 16);
icon!(icon_joystick, ICON_JOYSTICK, 16, 16);

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
