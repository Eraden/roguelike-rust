#[allow(unused_imports)]
use std::time::Duration;

use sdl2;
use sdl2::{Sdl, VideoSubsystem, TimerSubsystem};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};
use sdl2::pixels::Color;
use sdl2::render::TextureCreator;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::EventPump;
use sdl2::mouse::MouseButton;

use game::main_renderer::MainRenderer;
use game::managers::{FontManager, TextureManager};
use game::states::State;
use game::states::main_menu_state::*;
use game::states::first_map_state::*;
use game::config::Config;
use game::events::UpdateResult;

pub type WindowCanvas = Canvas<Window>;

pub struct App {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub canvas: WindowCanvas,
    pub config: Config,
    pub done: bool,
}

impl<'a> App {
    pub fn new() -> Self {
        let config = Config::new();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Rouge - Rust", config.width, config.height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .unwrap();

        App {
            sdl_context,
            video_subsystem,
            canvas,
            config,
            done: false
        }
    }

    pub fn init(&mut self) {
        self.clear();
    }

    pub fn main_loop(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let texture_creator = self.canvas.texture_creator();
        let font_context = sdl2::ttf::init().unwrap();
        let mut timer: TimerSubsystem = self.sdl_context.timer().unwrap();
        let sleep_time = Duration::new(0, 1_000_000_000u32 / 60);

        let mut main_renderer = MainRenderer::new(
            self.config.clone(),
            &font_context,
            &texture_creator,
        );
        let mut current_state = MainMenuState::new(&mut main_renderer);
        let mut _current_state = FirstMapState::new(&mut main_renderer);

        'running: loop {
            if self.done {
                break 'running
            }

            match self.handle_events(&mut event_pump, &mut current_state) {
                UpdateResult::Stop =>
                    break 'running,
                _ => {},
            }

            self.clear();
            {
                current_state.update(timer.ticks() as i32);
                current_state.render(&mut self.canvas, &mut main_renderer);
            }

            self.present();
            ::std::thread::sleep(sleep_time);
        }
    }

    fn handle_events(&mut self, event_pump: &mut EventPump, state: &mut State) -> UpdateResult {
        let mut res = UpdateResult::NoOp;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    return UpdateResult::Stop,
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, ..  } =>
                    return state.handle_click(&event),
                _ => UpdateResult::NoOp,
            };
        }
        res
    }

    fn present(&mut self) {
        self.canvas.present();
    }

    fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
    }
}
