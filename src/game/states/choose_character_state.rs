use game::sprites::Sprite;

use game::app::WindowCanvas;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use game::states::State;
use sdl2::event::Event;
use game::sprites::player_sprite::*;
use game::sprites::ClickHandler;

struct WarriorSpriteButton<'a> {
    warrior: WarriorSprite<'a>,
}

impl<'a> WarriorSpriteButton<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        use game::sprites::RenderPosition;

        let mut warrior = WarriorSprite::new(main_renderer);
        warrior.resize(&128);
        warrior.render_on(&0, &1);

        WarriorSpriteButton { warrior }
    }
}

impl<'a> Sprite<'a> for WarriorSpriteButton<'a> {
    fn update(&mut self, ticks: i32) {
        self.warrior.update(ticks);
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.warrior.render(canvas, main_renderer);
    }
}

impl<'a> ClickHandler for WarriorSpriteButton<'a> {
    fn handle_click(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }
}

pub struct ChooseCharacterState<'a> {
    warrior: WarriorSpriteButton<'a>,
    wizard: WizardSprite<'a>,
    rogue: RogueSprite<'a>,
    ranger: RangerSprite<'a>,
}

impl<'a> ChooseCharacterState<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        use game::sprites::RenderPosition;

        let warrior = WarriorSpriteButton::new(main_renderer);
        let mut wizard = WizardSprite::new(main_renderer);
        wizard.resize(&128);
        wizard.render_on(&1, &1);
        let mut rogue = RogueSprite::new(main_renderer);
        rogue.resize(&128);
        rogue.render_on(&2, &1);
        let mut ranger = RangerSprite::new(main_renderer);
        ranger.resize(&128);
        ranger.render_on(&3, &1);

        ChooseCharacterState {
            warrior,
            wizard,
            rogue,
            ranger
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
