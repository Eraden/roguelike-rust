use game::sprites::Sprite;

use game::app::WindowCanvas;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use game::states::State;
use sdl2::event::Event;
use game::sprites::warrior_sprite::*;

pub struct ChooseCharacterState<'a> {
    warrior: WarriorSprite<'a>,
}

impl<'a> ChooseCharacterState<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        use game::sprites::RenderPosition;

        let mut warrior = WarriorSprite::new(main_renderer);
        warrior.resize(&64);
        warrior.render_on(&1, &2);

        ChooseCharacterState {
            warrior,
        }
    }
}

impl<'a> State<'a> for ChooseCharacterState<'a> {
    fn update(&mut self, ticks: i32) {
        self.warrior.update(ticks);
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.warrior.render(canvas, main_renderer);
    }

    fn handle_click(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }

    fn handle_key_down(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }
}
