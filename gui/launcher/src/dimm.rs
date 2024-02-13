#![allow(dead_code)]
#![allow(unused_variables)]

use crate::scaling;

pub const HEIGHT : i32 = 450;
pub const WIDTH  : i32 = 300;
pub const BORDER : i32 = 10;

pub const HEIGHT_BUTTON_WIDE : i32 = 30;
pub const WIDTH_BUTTON_WIDE  : i32 = HEIGHT_BUTTON_WIDE*2;

pub const HEIGHT_BUTTON_REC : i32 = HEIGHT_BUTTON_WIDE;
pub const WIDTH_BUTTON_REC  : i32 = WIDTH_BUTTON_WIDE/2;

pub const HEIGHT_TEXT : i32 = 14;

pub const HEIGHT_BAR : i32 = HEIGHT_BUTTON_WIDE + BORDER;

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

function_scale!(bar, HEIGHT_BAR);

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
