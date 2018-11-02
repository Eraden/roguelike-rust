use game::sprites::Sprite;

use game::app::WindowCanvas;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use game::sprites::map_sprite::MapSprite;
use game::states::State;
use sdl2::event::Event;

pub struct ChooseCharacterState<'a> {
}

impl<'a> ChooseCharacterState<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        ChooseCharacterState {}
    }
}

impl<'a> State<'a> for ChooseCharacterState<'a> {
    fn update(&mut self, ticks: i32) {
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
    }

    fn handle_click(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }

    fn handle_key_down(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }
}
