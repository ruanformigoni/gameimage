use std::env;

use fltk::prelude::*;
use fltk::{
  app::Sender,
  enums,
  frame::Frame,
  image::SharedImage,
};

use shared::dimm;
use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::{hover_blink,column,row,fixed};

use crate::common;
use crate::games;
use common::Msg;

// fn: new {{{
pub fn new(tx : Sender<Msg>)
{
  let show_btn_game: bool = games::games().map(|e| e.len() > 1).unwrap_or(false);
  let show_btn_executable: bool = crate::frame::selector_executable::get_menu_entries().map(|e| e.0.len() > 1).unwrap_or(false);
  // Layout
  let mut frame_background = Frame::default_fill();
  if let Ok(env_image_launcher) = env::var("GIMG_LAUNCHER_IMG")
  && let Ok(shared_image) = SharedImage::load(env_image_launcher)
  {
    frame_background.set_image_scaled(Some(shared_image.clone()));
    frame_background.resize_callback(move |s,_,_,_,_| { s.set_image_scaled(Some(shared_image.clone())); });
  } // if
  else
  {
    println!("Failed to set launcher image");
  } // else
  // Buttons
  column!(col,
    col.add(&Frame::default());
    row!(row,
      row.set_margin(dimm::border_half());
      fixed!(row, btn_menu, shared::fltk::button::rect::list(), dimm::width_button_rec());
      if show_btn_game
      {
        row.add(&Frame::default());
        fixed!(row, btn_game, shared::fltk::button::rect::joystick(), dimm::width_button_rec());
        btn_game.clone().emit(tx, Msg::DrawSelectorGame);
        hover_blink!(btn_game);
      } // if
      if show_btn_executable
      {
        row.add(&Frame::default());
        fixed!(row, btn_executable, shared::fltk::button::rect::switch(), dimm::width_button_rec());
        btn_executable.clone().emit(tx, Msg::DrawSelectorExecutable);
        hover_blink!(btn_executable);
      } // if
      row.add(&Frame::default());
      fixed!(row, btn_play, shared::fltk::button::rect::play().with_color(enums::Color::Blue), dimm::width_button_rec());
    );
    col.fixed(&row, dimm::height_button_rec() + dimm::border());
  );

  let mut fb: Vec<u8> = vec![0u8; (dimm::width_launcher() * dimm::height_launcher() * 4) as usize];
  // Fill with required color
  for (_, pixel) in fb.chunks_exact_mut(4).enumerate() {
    pixel.copy_from_slice(&[0, 0, 0, 96]);
  }
  let image = fltk::image::RgbImage::new(&fb, dimm::width_launcher(), dimm::height_button_rec() + dimm::border(), enums::ColorDepth::Rgba8).unwrap();
  // Bottom background
  let mut row = row.clone();
  row.set_align(enums::Align::Inside | enums::Align::Center);
  row.set_frame(enums::FrameType::NoBox);
  row.set_image(Some(image));
  row.resize_callback(move |s,_,_,w,_|
  {
    let image = fltk::image::RgbImage::new(&fb, w, dimm::height_button_rec() + dimm::border(), enums::ColorDepth::Rgba8).unwrap();
    s.set_image(Some(image));
  });

  // Button left aligned
  btn_menu.clone().emit(tx, Msg::DrawMenu);
  hover_blink!(btn_menu);
  hover_blink!(btn_play);

  // Button right aligned
  let clone_tx = tx.clone();
  let mut clone_frame_background = frame_background.clone();
  btn_play.clone().set_callback(move |_|
  {
    // Cover image black and white
    if let Ok(env_image_launcher) = env::var("GIMG_LAUNCHER_IMG_GRAYSCALE")
    && let Ok(shared_image) = SharedImage::load(env_image_launcher)
    {
      clone_frame_background.set_image_scaled(Some(shared_image));
    } // if
    else
    {
      println!("Failed to set launcher image");
    } // else
    fltk::app::redraw();
    // Deactivate window
    clone_tx.send_awake(common::Msg::WindDeactivate);
    // Reference to spawned process
    std::thread::spawn(move ||
    {
      // Launch game
      games::launch();
      // Redraw
      clone_tx.send_activate(Msg::DrawCover);
    });
  });
} // fn: new }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
