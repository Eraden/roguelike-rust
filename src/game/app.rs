#[allow(unused_imports)]
use std::time::Duration;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::{Sdl, TimerSubsystem, VideoSubsystem};

use game::config::Config;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use game::states::first_map_state::*;
use game::states::main_menu_state::*;
use game::states::choose_character_state::ChooseCharacterState;
use game::states::State;

pub type WindowCanvas = Canvas<Window>;

pub struct App {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub canvas: WindowCanvas,
    pub config: Config,
}

pub enum AppState<'a> {
    MainMenu(MainMenuState<'a>),
    FirstMap(FirstMapState<'a>),
    ChooseCharacter(ChooseCharacterState<'a>),
}

impl<'a> App {
    pub fn new() -> Self {
        let config = Config::new();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Rogue - Rust", config.width, config.height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().accelerated().build().unwrap();

        App {
            sdl_context,
            video_subsystem,
            canvas,
            config,
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

        let mut main_renderer =
            MainRenderer::new(self.config.clone(), &font_context, &texture_creator);
        let mut app_state: AppState = AppState::MainMenu(MainMenuState::new(&mut main_renderer));

        'running: loop {
            match self.handle_events(&mut event_pump, &mut app_state) {
                UpdateResult::Stop => break 'running,
                UpdateResult::StartFirstMap => {
                    app_state = AppState::FirstMap(FirstMapState::new(&mut main_renderer))
                },
                UpdateResult::PickCharacter => {
                    app_state = AppState::ChooseCharacter(ChooseCharacterState::new(&mut main_renderer));
                },
                _ => {}
            }

            self.clear();

            match app_state {
                AppState::MainMenu(ref mut menu) => {
                    menu.update(timer.ticks() as i32);
                    menu.render(&mut self.canvas, &mut main_renderer);
                },
                AppState::FirstMap(ref mut map) => {
                    map.update(timer.ticks() as i32);
                    map.render(&mut self.canvas, &mut main_renderer);
                },
                AppState::ChooseCharacter(ref mut state) =>  {
                    state.update(timer.ticks() as i32);
                    state.render(&mut self.canvas, &mut main_renderer);
                },
            };

            self.present();
            ::std::thread::sleep(sleep_time);
        }
    }

    fn handle_events(
        &mut self,
        event_pump: &mut EventPump,
        app_state: &mut AppState,
    ) -> UpdateResult {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return UpdateResult::Stop,
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => match app_state {
                    AppState::MainMenu(state) => return state.handle_click(&event),
                    AppState::FirstMap(state) => return state.handle_click(&event),
                    AppState::ChooseCharacter(state) => return state.handle_click(&event),
                },
                _ => {}
            };
        }
        UpdateResult::NoOp
    }

    fn present(&mut self) {
        self.canvas.present();
    }

    fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
    }
}
