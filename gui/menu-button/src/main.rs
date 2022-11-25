use std::env;

use fltk::{enums::*, prelude::*, *};
use fltk_theme::{ColorTheme, color_themes};

fn main() {
    let border = 20;

    let args: Vec<String> = env::args().collect();

    if args.len() != 3
    {
      return
    }

    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();
    app::foreground(230,230,230);
    let color = Color::from_hex_str("#5294e2").unwrap().to_rgb();
    app::set_selection_color(color.0, color.1, color.2);

    let mut win = window::Window::default()
      .with_label("Dialog")
      .with_size(300, 100)
      .center_screen();

    let mut frame = frame::Frame::default()
      .size_of_parent()
      .center_of_parent();
    frame.set_frame(FrameType::NoBox);

    let mut menu = menu::MenuButton::default()
      .with_size(frame.width()-border*2, 30)
      .with_label(&args[1])
      .center_of(&frame);
    menu.add_choice(&args[2]);
    menu.set_callback(|m| { println!("{}", m.choice().unwrap()); app::quit(); });

    win.end();
    win.show();

    app.run().unwrap();
}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
