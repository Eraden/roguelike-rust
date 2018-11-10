use game::sprites::Sprite;

use game::app::WindowCanvas;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use game::sprites::choose_character_button_sprite::*;
use game::sprites::ClickHandler;
use game::sprites::*;
use game::states::State;
use sdl2::event::Event;

pub struct ChooseCharacterState<'a> {
    female_warrior: ChooseCharacterButtonSprite<'a>,
    male_warrior: ChooseCharacterButtonSprite<'a>,
    female_wizard: ChooseCharacterButtonSprite<'a>,
    male_wizard: ChooseCharacterButtonSprite<'a>,
    female_rogue: ChooseCharacterButtonSprite<'a>,
    male_rogue: ChooseCharacterButtonSprite<'a>,
    female_ranger: ChooseCharacterButtonSprite<'a>,
    male_ranger: ChooseCharacterButtonSprite<'a>,
    pub chosen_character: PlayerClass,
    pub chosen_gender: Gender,
}

impl<'a> ChooseCharacterState<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        use game::sprites::RenderPosition;

        // Female
        let mut female_warrior =
            ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Warrior);
        female_warrior.set_gender(Gender::Female);
        female_warrior.render_on(&1, &2);

        let mut female_wizard =
            ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Wizard);
        female_wizard.set_gender(Gender::Female);
        female_wizard.render_on(&2, &2);

        let mut female_rogue = ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Rogue);
        female_rogue.set_gender(Gender::Female);
        female_rogue.render_on(&3, &2);

        let mut female_ranger =
            ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Ranger);
        female_ranger.set_gender(Gender::Female);
        female_ranger.render_on(&4, &2);

        // Male
        let mut male_warrior =
            ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Warrior);
        male_warrior.set_gender(Gender::Male);
        male_warrior.render_on(&1, &3);

        let mut male_wizard = ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Wizard);
        male_wizard.set_gender(Gender::Male);
        male_wizard.render_on(&2, &3);

        let mut male_rogue = ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Rogue);
        male_rogue.set_gender(Gender::Male);
        male_rogue.render_on(&3, &3);

        let mut male_ranger = ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Ranger);
        male_ranger.set_gender(Gender::Male);
        male_ranger.render_on(&4, &3);

        ChooseCharacterState {
            female_warrior,
            female_wizard,
            female_rogue,
            female_ranger,
            male_warrior,
            male_wizard,
            male_rogue,
            male_ranger,
            chosen_character: PlayerClass::NoCharacter,
            chosen_gender: Gender::Female,
        }
    }

    pub fn on_update(&mut self) {
        let chosen = (&self.chosen_character, &self.chosen_gender);
        // Female
        self.female_warrior.on_update(chosen);
        self.female_wizard.on_update(chosen);
        self.female_rogue.on_update(chosen);
        self.female_ranger.on_update(chosen);
        // Male
        self.male_warrior.on_update(chosen);
        self.male_wizard.on_update(chosen);
        self.male_rogue.on_update(chosen);
        self.male_ranger.on_update(chosen);
    }
}

impl<'a> State<'a> for ChooseCharacterState<'a> {
    fn update(&mut self, ticks: i32) {
        // Female
        self.female_warrior.update(ticks);
        self.female_wizard.update(ticks);
        self.female_rogue.update(ticks);
        self.female_ranger.update(ticks);
        // Male
        self.male_warrior.update(ticks);
        self.male_wizard.update(ticks);
        self.male_rogue.update(ticks);
        self.male_ranger.update(ticks);
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        // Female
        self.female_warrior.render(canvas, main_renderer);
        self.female_wizard.render(canvas, main_renderer);
        self.female_rogue.render(canvas, main_renderer);
        self.female_ranger.render(canvas, main_renderer);
        // Male
        self.male_warrior.render(canvas, main_renderer);
        self.male_wizard.render(canvas, main_renderer);
        self.male_rogue.render(canvas, main_renderer);
        self.male_ranger.render(canvas, main_renderer);
    }

    fn handle_click(&mut self, event: &Event) -> UpdateResult {
        let res = match *event {
            Event::MouseButtonDown { x, y, .. } =>
            // Female
            {
                if self.female_warrior.check_is_inside(&x, &y) {
                    self.female_warrior.handle_click(event)
                } else if self.female_wizard.check_is_inside(&x, &y) {
                    self.female_wizard.handle_click(event)
                } else if self.female_ranger.check_is_inside(&x, &y) {
                    self.female_ranger.handle_click(event)
                } else if self.female_rogue.check_is_inside(&x, &y) {
                    self.female_rogue.handle_click(event)
                // Male
                } else if self.male_warrior.check_is_inside(&x, &y) {
                    self.male_warrior.handle_click(event)
                } else if self.male_wizard.check_is_inside(&x, &y) {
                    self.male_wizard.handle_click(event)
                } else if self.male_ranger.check_is_inside(&x, &y) {
                    self.male_ranger.handle_click(event)
                } else if self.male_rogue.check_is_inside(&x, &y) {
                    self.male_rogue.handle_click(event)
                } else {
                    UpdateResult::NoOp
                }
            }
            _ => UpdateResult::NoOp,
        };
        match res {
            UpdateResult::PlayerCharacterClicked(ref player_character, ref gender) => {
                self.chosen_character = player_character.clone();
                self.chosen_gender = gender.clone();
                self.on_update();
            }
            _ => (),
        };
        res
    }

    fn handle_key_down(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }
}
