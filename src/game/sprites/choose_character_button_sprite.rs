use game::app::WindowCanvas;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use game::sprites::player_sprite::*;
use game::sprites::*;
use sdl2::rect::Rect;

#[derive(Debug, PartialEq, Clone)]
enum ButtonState {
    Normal,
    Selected,
    MouseOver,
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

    pub fn change_state(&mut self, state: ButtonState) {
        self.state = state;
        match self.state {
            ButtonState::Selected => {
                self.animatable.animate(&0, 1, &mut self.renderable.source);
                self.renderable.source.set_y(self.animatable.tile_size);
            }
            ButtonState::MouseOver => {
                self.animatable.animate(&0, 2, &mut self.renderable.source);
                self.renderable.source.set_y(self.animatable.tile_size * 2);
            }
            ButtonState::Normal => {
                self.animatable.animate(&0, 0, &mut self.renderable.source);
                self.renderable.source.set_y(0);
            }
            _ => {}
        }
    }

    pub fn is_selected(&self) -> bool {
        self.state == ButtonState::Selected
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

impl<'a> Sprite<'a> for ChooseCharacterButtonBackground<'a> {
    fn update(&mut self, _ticks: i32) {}

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.renderable.render(canvas, main_renderer)
    }
}

pub struct ChooseCharacterButtonSprite<'a> {
    character_sprite: PlayerCharacterSprite<'a>,
    background: ChooseCharacterButtonBackground<'a>,
    player_class: PlayerClass,
}

impl<'a> ChooseCharacterButtonSprite<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>, player_class: PlayerClass) -> Self {
        let background: ChooseCharacterButtonBackground =
            ChooseCharacterButtonBackground::new(main_renderer);
        let mut character_sprite: PlayerCharacterSprite = match player_class {
            PlayerClass::Warrior => PlayerCharacterSprite::new_warrior(main_renderer),
            PlayerClass::Wizard => PlayerCharacterSprite::new_wizard(main_renderer),
            PlayerClass::Rogue => PlayerCharacterSprite::new_rogue(main_renderer),
            PlayerClass::Ranger => PlayerCharacterSprite::new_ranger(main_renderer),
        };
        character_sprite.resize(&128);
        character_sprite.move_by(6, 6);
        Self {
            background,
            character_sprite,
            player_class,
        }
    }

    pub fn check_is_inside(&self, x: &i32, y: &i32) -> bool {
        check_is_inside(x, y, &self.background.renderable.dest)
    }

    pub fn on_update(&mut self, chosen: (&PlayerClass, &Gender)) -> UpdateResult {
        match (*chosen.0 == self.player_class, *chosen.1 == self.character_sprite.gender) {
            (true, true) => self.background.change_state(ButtonState::Selected),
            _ => (),
        }
        UpdateResult::NoOp
    }

    pub fn set_gender(&mut self, gender: Gender) {
        self.character_sprite.set_gender(gender);
    }

    pub fn get_type(&self) -> (PlayerClass, Gender) {
        (
            self.player_class.clone(),
            self.character_sprite.gender.clone(),
        )
    }

    pub fn is_selected(&self) -> bool {
        self.background.is_selected()
    }

    pub fn set_normal(&mut self) {
        self.background.change_state(ButtonState::Normal);
    }

    pub fn set_mouse_above(&mut self) {
        self.background.change_state(ButtonState::MouseOver);
    }
}

impl<'a> RenderPosition for ChooseCharacterButtonSprite<'a> {
    fn render_on(&mut self, x: &usize, y: &usize) {
        self.background.render_on(x, y);
        let bd: Rect = { self.background.renderable.dest.clone() };
        self.character_sprite.move_to(bd.x() + 6, bd.y() + 6);
    }

    fn move_by(&mut self, x: i32, y: i32) {
        self.background.move_by(x, y);
        self.character_sprite.move_by(x, y);
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.background.move_to(x, y);
        self.character_sprite.move_to(x, y);
    }
}

impl<'a> Sprite<'a> for ChooseCharacterButtonSprite<'a> {
    fn update(&mut self, ticks: i32) {
        self.background.update(ticks);
        self.character_sprite.update(ticks);
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.background.render(canvas, main_renderer);
        self.character_sprite.render(canvas, main_renderer);
    }
}
