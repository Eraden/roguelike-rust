use game::app::WindowCanvas;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use game::sprites::player_sprite::*;
use game::sprites::*;
use sdl2::event::Event;
use sdl2::rect::Rect;

enum ButtonState {
    Normal,
    Selected,
}

struct ChooseCharacterButtonBackground<'a> {
    renderable: Renderable<'a>,
    animatable: Animatable,
    state: ButtonState,
}

impl<'a> ChooseCharacterButtonBackground<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let renderable: Renderable = Renderable::new(
            main_renderer,
            "./assets/textures/character_button_background.png",
            140,
            Rect::new(0, 0, 140, 140),
            Rect::new(0, 0, 140, 140),
        );
        let animatable: Animatable = Animatable::new(1, 140);
        Self {
            renderable,
            animatable,
            state: ButtonState::Normal,
        }
    }

    pub fn change_selected(&mut self, b: bool) {
        self.state = match b {
            true => ButtonState::Selected,
            false => ButtonState::Normal,
        };
        match self.state {
            ButtonState::Selected => {
                self.animatable.animate(&0, 1, &mut self.renderable.source);
                self.renderable.source.set_y(self.animatable.tile_size);
            }
            ButtonState::Normal => {
                self.animatable.animate(&0, 0, &mut self.renderable.source);
                self.renderable.source.set_y(0);
            }
        }
        let _x_ = 1;
    }
}

impl<'a> RenderPosition for ChooseCharacterButtonBackground<'a> {
    fn render_on(&mut self, x: &usize, y: &usize) {
        let tile_size: usize = self.renderable.dest_size() as usize;
        render_on(&mut self.renderable.dest, &tile_size, x, y);
    }

    fn move_by(&mut self, x: i32, y: i32) {
        let dest: &mut Rect = &mut self.renderable.dest;
        let c = { dest.clone() };
        let (dx, dy) = { (c.x(), c.y()) };
        dest.set_x(dx + x);
        dest.set_y(dy + y);
    }

    fn move_to(&mut self, x: i32, y: i32) {
        let dest = &mut self.renderable.dest;
        dest.set_x(x);
        dest.set_y(y);
    }
}

impl<'a> ClickHandler for ChooseCharacterButtonBackground<'a> {
    fn handle_click(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }
}

impl<'a> Sprite<'a> for ChooseCharacterButtonBackground<'a> {
    fn update(&mut self, _ticks: i32) {}

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.renderable.render(canvas, main_renderer)
    }
}

pub struct ChooseCharacterButtonSprite<'a> {
    character_class: PlayerCharacterSprite<'a>,
    background: ChooseCharacterButtonBackground<'a>,
    player_class: PlayerClass,
}

impl<'a> ChooseCharacterButtonSprite<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>, player_class: PlayerClass) -> Self {
        let background: ChooseCharacterButtonBackground =
            ChooseCharacterButtonBackground::new(main_renderer);
        let mut character_class: PlayerCharacterSprite = match player_class {
            PlayerClass::Warrior => PlayerCharacterSprite::new_warrior(main_renderer),
            PlayerClass::Wizard => PlayerCharacterSprite::new_wizard(main_renderer),
            PlayerClass::Rogue => PlayerCharacterSprite::new_rogue(main_renderer),
            PlayerClass::Ranger => PlayerCharacterSprite::new_ranger(main_renderer),
            _ => panic!("Invalid character type"),
        };
        character_class.resize(&128);
        character_class.move_by(6, 6);
        Self {
            background,
            character_class,
            player_class,
        }
    }

    pub fn check_is_inside(&self, x: &i32, y: &i32) -> bool {
        check_is_inside(x, y, &self.background.renderable.dest)
    }

    pub fn on_update(&mut self, chosen: (&PlayerClass, &Gender)) -> UpdateResult {
        self.background
            .change_selected(
                *chosen.0 == self.player_class &&
                    *chosen.1 == self.character_class.gender
            );
        UpdateResult::NoOp
    }

    pub fn set_gender(&mut self, gender: Gender) {
        self.character_class.set_gender(gender);
    }
}

impl<'a> RenderPosition for ChooseCharacterButtonSprite<'a> {
    fn render_on(&mut self, x: &usize, y: &usize) {
        self.background.render_on(x, y);
        let bd: Rect = { self.background.renderable.dest.clone() };
        self.character_class.move_to(bd.x() + 6, bd.y() + 6);
    }

    fn move_by(&mut self, x: i32, y: i32) {
        self.background.move_by(x, y);
        self.character_class.move_by(x, y);
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.background.move_to(x, y);
        self.character_class.move_to(x, y);
    }
}

impl<'a> ClickHandler for ChooseCharacterButtonSprite<'a> {
    fn handle_click(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::PlayerCharacterClicked(
            self.player_class.clone(),
            self.character_class.gender.clone(),
        )
    }
}

impl<'a> Sprite<'a> for ChooseCharacterButtonSprite<'a> {
    fn update(&mut self, ticks: i32) {
        self.background.update(ticks);
        self.character_class.update(ticks);
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.background.render(canvas, main_renderer);
        self.character_class.render(canvas, main_renderer);
    }
}
