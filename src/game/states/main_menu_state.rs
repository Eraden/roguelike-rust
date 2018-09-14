use game::sprites::Sprite;
use game::sprites::start_button::*;
use game::sprites::quit_button::*;

use game::main_renderer::MainRenderer;
use game::states::State;
use game::app::WindowCanvas;
use game::events::UpdateResult;

use sdl2::event::Event;

pub struct MainMenuState<'a> {
    pub start_button: StartButtonSprite<'a>,
    pub quit_button: QuitButtonSprite<'a>,
}

impl<'a> MainMenuState<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let start_button = StartButtonSprite::new(main_renderer);
        let quit_button = QuitButtonSprite::new(main_renderer);
        MainMenuState { start_button, quit_button }
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
            Event::MouseButtonDown { x, y, .. } =>
                if self.quit_button.is_inside(&x, &y) {
                    UpdateResult::Stop
                } else if self.start_button.is_inside(&x, &y) {
                    UpdateResult::StartFirstMap
                } else {
                    UpdateResult::NoOp
                },
            _ => UpdateResult::NoOp,
        }
    }

    fn handle_key_down(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }
}
