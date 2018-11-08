use game::sprites::*;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use sdl2::rect::Rect;
use sdl2::event::Event;
use game::app::WindowCanvas;
use game::sprites::player_sprite::*;

struct ChooseCharacterButtonBackground<'a> {
    renderable: Renderable<'a>,
    animatable: Animatable,
}

impl<'a> ChooseCharacterButtonBackground<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let renderable = Renderable::new(
            main_renderer,
            "./assets/textures/character_button_background.png",
            140,
            Rect::new(0, 0, 140, 140),
            Rect::new(0, 0, 140, 140),
        );
        let animatable = Animatable::new(
            1,
            140,
        );
        Self { renderable, animatable }
    }
}

impl<'a> RenderPosition for ChooseCharacterButtonBackground<'a> {
    fn render_on(&mut self, x: &usize, y: &usize) {
        let tile_size: i32 = self.renderable.dest_size();
        render_on(&mut self.renderable.dest, &(tile_size as usize), x, y);
    }

    fn move_by(&mut self, x: i32, y: i32) {
        let dest = &mut self.renderable.dest;
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
    fn handle_click(&mut self, event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }
}

impl<'a> Sprite<'a> for ChooseCharacterButtonBackground<'a> {
    fn update(&mut self, ticks: i32) {}

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.renderable.render(canvas, main_renderer)
    }
}

pub struct ChooseCharacterButtonSprite<'a> {
    character_class: PlayerCharacterSprite<'a>,
    background: ChooseCharacterButtonBackground<'a>,
}

impl<'a> ChooseCharacterButtonSprite<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>, character_class: PlayerClass) -> Self {
        let background = ChooseCharacterButtonBackground::new(main_renderer);
        let mut character_class = match character_class {
            PlayerClass::Warrior => PlayerCharacterSprite::new_warrior(main_renderer),
            PlayerClass::Wizard => PlayerCharacterSprite::new_wizard(main_renderer),
            PlayerClass::Rogue => PlayerCharacterSprite::new_rogue(main_renderer),
            PlayerClass::Ranger => PlayerCharacterSprite::new_ranger(main_renderer),
        };
        character_class.resize(&128);
        character_class.move_by(6, 6);
        Self { background, character_class }
    }
}

impl<'a> RenderPosition for ChooseCharacterButtonSprite<'a> {
    fn render_on(&mut self, x: &usize, y: &usize) {
        self.background.render_on(x, y);
        let bd = { self.background.renderable.dest.clone() };
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
    fn handle_click(&mut self, event: &Event) -> UpdateResult {
        UpdateResult::NoOp
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
