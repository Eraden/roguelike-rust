use game::sprites::Sprite;

use game::app::WindowCanvas;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use game::sprites::button::ButtonSprite;
use game::sprites::choose_character_button_sprite::*;
use game::sprites::*;
use game::states::State;
use sdl2::event::Event;

const DEFAULT_MOVE_CHECK: i32 = 10;

pub struct ChooseCharacterState<'a> {
    female_warrior: ChooseCharacterButtonSprite<'a>,
    male_warrior: ChooseCharacterButtonSprite<'a>,
    female_wizard: ChooseCharacterButtonSprite<'a>,
    male_wizard: ChooseCharacterButtonSprite<'a>,
    female_rogue: ChooseCharacterButtonSprite<'a>,
    male_rogue: ChooseCharacterButtonSprite<'a>,
    female_ranger: ChooseCharacterButtonSprite<'a>,
    male_ranger: ChooseCharacterButtonSprite<'a>,
    continue_button: ButtonSprite<'a>,
    chosen: (PlayerClass, Gender),
    above: Option<(PlayerClass, Gender)>,
    move_check: i32,
}

impl<'a> ChooseCharacterState<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        use game::sprites::RenderPosition;
        let config = { main_renderer.config.clone() };
        //        let mut quit_button = ButtonSprite::new(main_renderer, "Quit", 0, (config.height - 200) as i32);
        //        quit_button.center_x(&config);
        let continue_button = ButtonSprite::new(
            main_renderer,
            "Continue",
            (config.width - 400) as i32,
            (config.height - 100) as i32,
        );

        let chosen_character = PlayerClass::Warrior;
        let chosen_gender = Gender::Female;

        // Female
        let mut female_warrior =
            ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Warrior);
        female_warrior.set_gender(Gender::Female);
        female_warrior.render_on(&1, &2);
        female_warrior.on_update((&chosen_character, &chosen_gender));

        let mut female_wizard =
            ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Wizard);
        female_wizard.set_gender(Gender::Female);
        female_wizard.render_on(&2, &2);
        female_wizard.on_update((&chosen_character, &chosen_gender));

        let mut female_rogue = ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Rogue);
        female_rogue.set_gender(Gender::Female);
        female_rogue.render_on(&3, &2);
        female_rogue.on_update((&chosen_character, &chosen_gender));

        let mut female_ranger =
            ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Ranger);
        female_ranger.set_gender(Gender::Female);
        female_ranger.render_on(&4, &2);
        female_ranger.on_update((&chosen_character, &chosen_gender));

        // Male
        let mut male_warrior =
            ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Warrior);
        male_warrior.set_gender(Gender::Male);
        male_warrior.render_on(&1, &3);
        male_warrior.on_update((&chosen_character, &chosen_gender));

        let mut male_wizard = ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Wizard);
        male_wizard.set_gender(Gender::Male);
        male_wizard.render_on(&2, &3);
        male_wizard.on_update((&chosen_character, &chosen_gender));

        let mut male_rogue = ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Rogue);
        male_rogue.set_gender(Gender::Male);
        male_rogue.render_on(&3, &3);
        male_rogue.on_update((&chosen_character, &chosen_gender));

        let mut male_ranger = ChooseCharacterButtonSprite::new(main_renderer, PlayerClass::Ranger);
        male_ranger.set_gender(Gender::Male);
        male_ranger.render_on(&4, &3);
        male_ranger.on_update((&chosen_character, &chosen_gender));

        ChooseCharacterState {
            female_warrior,
            female_wizard,
            female_rogue,
            female_ranger,
            male_warrior,
            male_wizard,
            male_rogue,
            male_ranger,
            continue_button,
            chosen: (chosen_character, chosen_gender),
            above: None,
            move_check: DEFAULT_MOVE_CHECK.clone(),
        }
    }

    pub fn chosen_character(&self) -> PlayerClass {
        self.chosen.0.clone()
    }

    pub fn chosen_gender(&self) -> Gender {
        self.chosen.1.clone()
    }

    pub fn on_update(&mut self) {}

    #[inline]
    pub fn is_female_warrior(&self, x: i32, y: i32) -> bool {
        self.female_warrior.check_is_inside(&x, &y)
    }

    #[inline]
    pub fn is_female_wizard(&self, x: i32, y: i32) -> bool {
        self.female_wizard.check_is_inside(&x, &y)
    }

    #[inline]
    pub fn is_female_ranger(&self, x: i32, y: i32) -> bool {
        self.female_ranger.check_is_inside(&x, &y)
    }

    #[inline]
    pub fn is_female_rogue(&self, x: i32, y: i32) -> bool {
        self.female_rogue.check_is_inside(&x, &y)
    }

    #[inline]
    pub fn is_male_warrior(&self, x: i32, y: i32) -> bool {
        self.male_warrior.check_is_inside(&x, &y)
    }

    #[inline]
    pub fn is_male_wizard(&self, x: i32, y: i32) -> bool {
        self.male_wizard.check_is_inside(&x, &y)
    }

    #[inline]
    pub fn is_male_ranger(&self, x: i32, y: i32) -> bool {
        self.male_ranger.check_is_inside(&x, &y)
    }

    #[inline]
    pub fn is_male_rogue(&self, x: i32, y: i32) -> bool {
        self.male_rogue.check_is_inside(&x, &y)
    }

    #[inline]
    pub fn is_continue_button(&self, x: i32, y: i32) -> bool {
        self.continue_button.is_inside(&x, &y)
    }

    fn button_with_type(
        &mut self,
        button_type: &(PlayerClass, Gender),
    ) -> &mut ChooseCharacterButtonSprite<'a> {
        match button_type.clone() {
            (PlayerClass::Warrior, Gender::Female) => &mut self.female_warrior,
            (PlayerClass::Wizard, Gender::Female) => &mut self.female_wizard,
            (PlayerClass::Ranger, Gender::Female) => &mut self.female_ranger,
            (PlayerClass::Rogue, Gender::Female) => &mut self.female_rogue,
            (PlayerClass::Warrior, Gender::Male) => &mut self.male_warrior,
            (PlayerClass::Wizard, Gender::Male) => &mut self.male_wizard,
            (PlayerClass::Ranger, Gender::Male) => &mut self.male_ranger,
            (PlayerClass::Rogue, Gender::Male) => &mut self.male_rogue,
        }
    }
}

impl<'a> State<'a> for ChooseCharacterState<'a> {
    fn update(&mut self, ticks: i32) {
        // Female
        self.female_warrior.update(ticks + 4);
        self.female_wizard.update(ticks + 2);
        self.female_rogue.update(ticks + 1);
        self.female_ranger.update(ticks + 3);
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

        self.continue_button.render(canvas, main_renderer);
    }

    fn handle_click(&mut self, event: &Event) -> UpdateResult {
        let res = match *event {
            Event::MouseButtonDown { x, y, .. } => match (x, y) {
                _female_warrior if self.is_female_warrior(x, y) => {
                    UpdateResult::PlayerCharacterClicked(self.female_warrior.get_type())
                }
                _female_wizard if self.is_female_wizard(x, y) => {
                    UpdateResult::PlayerCharacterClicked(self.female_wizard.get_type())
                }
                _female_ranger if self.is_female_ranger(x, y) => {
                    UpdateResult::PlayerCharacterClicked(self.female_ranger.get_type())
                }
                _female_rogue if self.is_female_rogue(x, y) => {
                    UpdateResult::PlayerCharacterClicked(self.female_rogue.get_type())
                }
                _male_warrior if self.is_male_warrior(x, y) => {
                    UpdateResult::PlayerCharacterClicked(self.male_warrior.get_type())
                }
                _male_wizard if self.is_male_wizard(x, y) => {
                    UpdateResult::PlayerCharacterClicked(self.male_wizard.get_type())
                }
                _male_ranger if self.is_male_ranger(x, y) => {
                    UpdateResult::PlayerCharacterClicked(self.male_ranger.get_type())
                }
                _male_rogue if self.is_male_rogue(x, y) => {
                    UpdateResult::PlayerCharacterClicked(self.male_rogue.get_type())
                }
                _continue_button if self.is_continue_button(x, y) => UpdateResult::NoOp,
                _ => UpdateResult::NoOp,
            },
            _ => UpdateResult::NoOp,
        };
        match res {
            UpdateResult::PlayerCharacterClicked((ref player_character, ref gender)) => {
                let current = (player_character.clone(), gender.clone());
                let old = self.chosen.clone();
                if self.chosen != current {
                    self.button_with_type(&old).set_normal();
                    self.button_with_type(&current).set_selected();
                    self.chosen = current;
                }
            }
            _ => (),
        };
        res
    }

    fn handle_key_down(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }

    fn handle_mouse_move(&mut self, event: &Event) -> UpdateResult {
        if self.move_check > 0 {
            self.move_check -= 1;
            return UpdateResult::NoOp;
        }
        self.move_check = DEFAULT_MOVE_CHECK.clone();
        let res = match event {
            Event::MouseMotion { x, y, .. } => {
                {
                    let old = self.chosen.clone();
                    if self.button_with_type(&old).check_is_inside(x, y) {
                        return UpdateResult::NoOp;
                    }
                }
                match (x, y) {
                    _female_warrior if self.is_female_warrior(x.clone(), y.clone()) => {
                        UpdateResult::AboveButton(self.female_warrior.get_type())
                    }
                    _female_wizard if self.is_female_wizard(x.clone(), y.clone()) => {
                        UpdateResult::AboveButton(self.female_wizard.get_type())
                    }
                    _female_ranger if self.is_female_ranger(x.clone(), y.clone()) => {
                        UpdateResult::AboveButton(self.female_ranger.get_type())
                    }
                    _female_rogue if self.is_female_rogue(x.clone(), y.clone()) => {
                        UpdateResult::AboveButton(self.female_rogue.get_type())
                    }
                    _male_warrior if self.is_male_warrior(x.clone(), y.clone()) => {
                        UpdateResult::AboveButton(self.male_warrior.get_type())
                    }
                    _male_wizard if self.is_male_wizard(x.clone(), y.clone()) => {
                        UpdateResult::AboveButton(self.male_wizard.get_type())
                    }
                    _male_ranger if self.is_male_ranger(x.clone(), y.clone()) => {
                        UpdateResult::AboveButton(self.male_ranger.get_type())
                    }
                    _male_rogue if self.is_male_rogue(x.clone(), y.clone()) => {
                        UpdateResult::AboveButton(self.male_rogue.get_type())
                    }
                    _continue_button if self.continue_button.is_inside(&x, &y) => {
                        UpdateResult::NoOp
                    }
                    _ => UpdateResult::NoOp,
                }
            }
            _ => return UpdateResult::NoOp,
        };
        match res {
            UpdateResult::AboveButton(ref button_type) => {
                if self.above == Some(button_type.clone()) {
                    return UpdateResult::NoOp;
                }
                if let Some(old_button) = self.above.clone() {
                    let mut button = self.button_with_type(&old_button);
                    if !button.is_selected() {
                        button.set_normal();
                    }
                }
                self.button_with_type(button_type).set_mouse_above();
                self.above = Some(button_type.clone());
            }
            _ => {
                if let Some(old_button) = self.above.clone() {
                    self.button_with_type(&old_button).set_normal();
                }
                self.above = None;
            }
        }
        res
    }
}
