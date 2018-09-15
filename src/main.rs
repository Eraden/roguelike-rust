#![feature(type_ascription)]
#![feature(range_contains)]
extern crate sdl2;

pub mod game;

fn main() {
//    println!("Map\n{:?}", game::map::load_map(&"first_map".to_string()));

    let mut app = game::app::App::new();
    app.init();
    app.main_loop();
}
