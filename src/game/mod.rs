pub mod app;
pub mod config;
pub mod events;
pub mod main_renderer;
pub mod managers;
pub mod map;
pub mod sprites;
pub mod states;
pub mod ui;

#[derive(Debug, PartialEq, Clone)]
pub struct MousePosition {
    x: u32,
    y: u32,
}
