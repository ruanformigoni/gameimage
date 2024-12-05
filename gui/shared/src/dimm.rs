const HEIGHT_WIZARD : i32 = 500;
const WIDTH_WIZARD  : i32 = 500;
const HEIGHT_LAUNCHER : i32 = 450;
const WIDTH_LAUNCHER  : i32 = 300;
const BORDER : i32 = 10;
const BORDER_HALF : i32 = BORDER / 2;

const HEIGHT_BUTTON_WIDE : i32 = 30;
const WIDTH_BUTTON_WIDE  : i32 = HEIGHT_BUTTON_WIDE*2;

const HEIGHT_BUTTON_REC : i32 = HEIGHT_BUTTON_WIDE;
const WIDTH_BUTTON_REC  : i32 = WIDTH_BUTTON_WIDE/2;

const HEIGHT_TEXT : i32 = 14;

const HEIGHT_STATUS : i32 = (HEIGHT_WIZARD as f64 * 0.05) as i32;

const HEIGHT_SEP : i32 = 1;

const HEIGHT_HEADER : i32 = HEIGHT_BUTTON_WIDE + BORDER*2 + HEIGHT_SEP;
const HEIGHT_FOOTER : i32 = HEIGHT_BUTTON_WIDE + BORDER*2 + HEIGHT_STATUS;

const HEIGHT_BAR : i32 = HEIGHT_BUTTON_WIDE + BORDER;

const POSY_FOOTER : i32 = HEIGHT_WIZARD - HEIGHT_FOOTER;

macro_rules! function_scale
{
  ($func_name:ident, $baseline:expr) =>
  {
    pub fn $func_name() -> i32
    {
      return $baseline;
    }
  }
}

function_scale!(height_wizard, HEIGHT_WIZARD);
function_scale!(width_wizard, WIDTH_WIZARD);
function_scale!(height_launcher, HEIGHT_LAUNCHER);
function_scale!(width_launcher, WIDTH_LAUNCHER);
function_scale!(border, BORDER);
function_scale!(border_half, BORDER_HALF);

function_scale!(height_button_wide, HEIGHT_BUTTON_WIDE);
function_scale!(width_button_wide, WIDTH_BUTTON_WIDE);

function_scale!(height_button_rec, HEIGHT_BUTTON_REC);
function_scale!(width_button_rec, WIDTH_BUTTON_REC);

function_scale!(height_text_header, HEIGHT_TEXT*2);
function_scale!(height_text, HEIGHT_TEXT);

function_scale!(height_status, HEIGHT_STATUS);

function_scale!(height_sep, HEIGHT_SEP);

function_scale!(height_header, HEIGHT_HEADER);
function_scale!(height_footer, HEIGHT_FOOTER);

function_scale!(posy_footer, POSY_FOOTER);

function_scale!(bar, HEIGHT_BAR);

pub fn width_checkbutton() -> i32 { 18 }

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
