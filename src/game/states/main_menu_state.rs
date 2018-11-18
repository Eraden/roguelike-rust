use game::sprites::button::*;
use game::sprites::Sprite;

use game::app::WindowCanvas;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use game::states::State;

use sdl2::event::Event;

pub struct MainMenuState<'a> {
    start_button: ButtonSprite<'a>,
    quit_button: ButtonSprite<'a>,
}

impl<'a> MainMenuState<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let config = { main_renderer.config.clone() };
        let mut start_button =
            ButtonSprite::new(main_renderer, "Start", 0, (config.height - 350) as i32);
        start_button.center_x(&config);
        let mut quit_button =
            ButtonSprite::new(main_renderer, "Quit", 0, (config.height - 200) as i32);
        quit_button.center_x(&config);
        MainMenuState {
            start_button,
            quit_button,
        }
    }

    fn is_quit_button(&self, x: i32, y: i32) -> bool {
        self.quit_button.is_inside(&x, &y)
    }

    fn is_start_button(&self, x: i32, y: i32) -> bool {
        self.start_button.is_inside(&x, &y)
    }
}

impl<'a> State<'a> for MainMenuState<'a> {
    fn update(&mut self, ticks: i32) {
        self.start_button.update(ticks);
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.start_button.render(canvas, main_renderer);
        self.quit_button.render(canvas, main_renderer);
    }

    fn handle_click(&mut self, event: &Event) -> UpdateResult {
        match *event {
            Event::MouseButtonDown { x, y, .. } => match (x, y) {
                _start if self.is_start_button(x, y) => UpdateResult::PickCharacter,
                _quit if self.is_quit_button(x, y) => UpdateResult::Stop,
                _ => UpdateResult::NoOp,
            },
            _ => UpdateResult::NoOp,
        }
    }

    fn handle_key_down(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }

    fn handle_mouse_move(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }
}
