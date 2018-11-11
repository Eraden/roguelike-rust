use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::sprites::*;
use sdl2::rect::Rect;

const WARRIOR_TEXTURE: &'static str = "./assets/textures/warrior spritesheet calciumtrice.png";
const WIZARD_TEXTURE: &'static str = "./assets/textures/wizard spritesheet calciumtrice.png";
const ROGUE_TEXTURE: &'static str = "./assets/textures/rogue spritesheet calciumtrice.png";
const RANGER_TEXTURE: &'static str = "./assets/textures/ranger spritesheet calciumtrice.png";

fn resolve_texture(player_class: &PlayerClass) -> &str {
    match player_class {
        PlayerClass::Warrior => WARRIOR_TEXTURE,
        PlayerClass::Wizard => WIZARD_TEXTURE,
        PlayerClass::Rogue => ROGUE_TEXTURE,
        PlayerClass::Ranger => RANGER_TEXTURE,
    }
}

pub struct PlayerCharacterSprite<'a> {
    pub gender: Gender,
    animatable: Animatable,
    renderable: Renderable<'a>,
    #[allow(dead_code)]
    player_class: PlayerClass,
}

impl<'a> PlayerCharacterSprite<'a> {
    fn new(main_renderer: &mut MainRenderer<'a, 'a>, player_class: PlayerClass) -> Self {
        let config = { main_renderer.config.clone() };
        let spritesheet = resolve_texture(&player_class);
        Self {
            gender: Gender::Female,
            animatable: Animatable::new(5, 32),
            renderable: Renderable::new(
                main_renderer,
                spritesheet,
                32,
                Rect::new(0, 0, 32, 32),
                Rect::new(0, 0, config.render_tile.width, config.render_tile.height),
            ),
            player_class: player_class.clone(),
        }
    }

    pub fn new_warrior(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        PlayerCharacterSprite::new(main_renderer, PlayerClass::Warrior)
    }

    pub fn new_wizard(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        PlayerCharacterSprite::new(main_renderer, PlayerClass::Wizard)
    }

    pub fn new_rogue(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        PlayerCharacterSprite::new(main_renderer, PlayerClass::Rogue)
    }

    pub fn new_ranger(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        PlayerCharacterSprite::new(main_renderer, PlayerClass::Ranger)
    }

    pub fn resize(&mut self, size: &u32) {
        self.renderable.dest_size = size.clone() as i32;
        self.renderable.dest = Rect::new(0, 0, size.clone(), size.clone());
    }

    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }
}

impl<'a> AnimationRowResolver for PlayerCharacterSprite<'a> {
    fn resolve_y(&self, animation: &Animation) -> i32 {
        let animation_offset: i32 = match animation {
            Animation::Idle => 0,
            Animation::Running => 2,
            Animation::Attacking => 3,
            _ => 0,
        };
        let gender_offset: i32 = match self.gender {
            Gender::Male => 0,
            Gender::Female => 5,
        };
        animation_offset + gender_offset
    }
}

impl<'a> RenderPosition for PlayerCharacterSprite<'a> {
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

impl<'a> Sprite<'a> for PlayerCharacterSprite<'a> {
    fn update(&mut self, ticks: i32) {
        let y = { self.resolve_y(&self.animatable.animation) };
        self.animatable
            .animate(&ticks, y, &mut self.renderable.source);
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.renderable.render(canvas, main_renderer);
    }
}
