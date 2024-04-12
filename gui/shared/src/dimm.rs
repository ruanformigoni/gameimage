use crate::scaling;

pub const HEIGHT : i32 = 500;
pub const WIDTH  : i32 = 500;
pub const BORDER : i32 = 10;

pub const HEIGHT_BUTTON_WIDE : i32 = 30;
pub const WIDTH_BUTTON_WIDE  : i32 = HEIGHT_BUTTON_WIDE*2;

pub const HEIGHT_BUTTON_REC : i32 = HEIGHT_BUTTON_WIDE;
pub const WIDTH_BUTTON_REC  : i32 = WIDTH_BUTTON_WIDE/2;

pub const HEIGHT_TEXT : i32 = 14;

pub const HEIGHT_STATUS : i32 = (HEIGHT as f64 * 0.05) as i32;

pub const HEIGHT_SEP : i32 = 2;

pub const HEIGHT_HEADER : i32 = HEIGHT_BUTTON_WIDE + BORDER*2 + HEIGHT_SEP;
pub const HEIGHT_FOOTER : i32 = HEIGHT_BUTTON_WIDE + BORDER*2 + HEIGHT_STATUS;

pub const HEIGHT_BAR : i32 = HEIGHT_BUTTON_WIDE + BORDER;

pub const POSY_FOOTER : i32 = HEIGHT - HEIGHT_FOOTER;

macro_rules! function_scale
{
  ($func_name:ident, $baseline:expr) =>
  {
    pub fn $func_name() -> i32
    {
      return if let Some(factor) = scaling::factor() { ($baseline as f32 * factor) as i32 } else { $baseline }
    }
  }
}

function_scale!(height, HEIGHT);
function_scale!(width, WIDTH);
function_scale!(border, BORDER);

function_scale!(height_button_wide, HEIGHT_BUTTON_WIDE);
function_scale!(width_button_wide, WIDTH_BUTTON_WIDE);

function_scale!(height_button_rec, HEIGHT_BUTTON_REC);
function_scale!(width_button_rec, WIDTH_BUTTON_REC);

function_scale!(height_text, HEIGHT_TEXT);

function_scale!(height_status, HEIGHT_STATUS);

function_scale!(height_sep, HEIGHT_SEP);

function_scale!(height_header, HEIGHT_HEADER);
function_scale!(height_footer, HEIGHT_FOOTER);

function_scale!(posy_footer, POSY_FOOTER);

function_scale!(bar, HEIGHT_BAR);

pub fn width_checkbutton() -> i32 { 20 }

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
