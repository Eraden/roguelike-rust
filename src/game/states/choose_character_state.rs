use game::sprites::Sprite;

use game::app::WindowCanvas;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use game::sprites::*;
use game::sprites::player_sprite::*;
use game::sprites::choose_character_button_sprite::*;
use game::sprites::ClickHandler;
use game::states::State;
use sdl2::event::Event;

pub struct ChooseCharacterState<'a> {
    warrior: ChooseCharacterButtonSprite<'a>,
    wizard: ChooseCharacterButtonSprite<'a>,
    rogue: ChooseCharacterButtonSprite<'a>,
    ranger: ChooseCharacterButtonSprite<'a>,
}

impl<'a> ChooseCharacterState<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        use game::sprites::RenderPosition;

        let mut warrior = ChooseCharacterButtonSprite::new(
            main_renderer,
            PlayerClass::Warrior,
        );
        warrior.render_on(&1, &2);
        let mut wizard = ChooseCharacterButtonSprite::new(
            main_renderer,
            PlayerClass::Wizard,
        );
        wizard.render_on(&2, &2);
        let mut rogue = ChooseCharacterButtonSprite::new(
            main_renderer,
            PlayerClass::Rogue,
        );
        rogue.render_on(&3, &2);
        let mut ranger = ChooseCharacterButtonSprite::new(
            main_renderer,
            PlayerClass::Ranger,
        );
        ranger.render_on(&4, &2);

        ChooseCharacterState {
            warrior,
            wizard,
            rogue,
            ranger,
        }
    }
}

impl<'a> State<'a> for ChooseCharacterState<'a> {
    fn update(&mut self, ticks: i32) {
        self.warrior.update(ticks);
        self.wizard.update(ticks);
        self.rogue.update(ticks);
        self.ranger.update(ticks);
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.warrior.render(canvas, main_renderer);
        self.wizard.render(canvas, main_renderer);
        self.rogue.render(canvas, main_renderer);
        self.ranger.render(canvas, main_renderer);
    }

    fn handle_click(&mut self, event: &Event) -> UpdateResult {
        self.warrior.handle_click(event)
    }

    fn handle_key_down(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }
}
