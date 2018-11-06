use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::sprites::*;
use sdl2::rect::Rect;

pub struct PlayerCharacterSprite<'a> {
    pub gender: Gender,
    renderable: Renderable<'a>,
    animatable: Animatable,
}

impl<'a> PlayerCharacterSprite<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>, spritesheet: &str) -> Self {
        let config = { main_renderer.config.clone() };
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
        }
    }

    pub fn new_warrior(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        PlayerCharacterSprite::new(
            main_renderer,
            "./assets/textures/warrior spritesheet calciumtrice.png",
        )
    }

    pub fn new_wizard(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        PlayerCharacterSprite::new(
            main_renderer,
            "./assets/textures/wizard spritesheet calciumtrice.png",
        )
    }

    pub fn new_rogue(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        PlayerCharacterSprite::new(
            main_renderer,
            "./assets/textures/rogue spritesheet calciumtrice.png",
        )
    }

    pub fn new_ranger(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        PlayerCharacterSprite::new(
            main_renderer,
            "./assets/textures/ranger spritesheet calciumtrice.png",
        )
    }

    pub fn resize(&mut self, size: &u32) {
        self.renderable.dest_size = size.clone() as i32;
        self.renderable.dest = Rect::new(0, 0, size.clone(), size.clone());
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

macro_rules! player_sprite {
    ($class_name: ident, $builder: expr) => {
        pub struct $class_name<'a> {
            player_character: PlayerCharacterSprite<'a>,
        }

        impl<'a> $class_name<'a> {
            pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
                Self {
                    player_character: $builder(main_renderer),
                }
            }

            pub fn resize(&mut self, size: &u32) {
                self.player_character.resize(size);
            }
        }

        impl<'a> RenderPosition for $class_name<'a> {
            fn render_on(&mut self, x: &usize, y: &usize) {
                self.player_character.render_on(x, y);
            }
        }

        impl<'a> Sprite<'a> for $class_name<'a> {
            fn update(&mut self, ticks: i32) {
                self.player_character.update(ticks);
            }

            fn render(
                &mut self,
                canvas: &mut WindowCanvas,
                main_renderer: &mut MainRenderer<'a, 'a>,
            ) {
                self.player_character.render(canvas, main_renderer);
            }
        }
    };
}

player_sprite!(WarriorSprite, PlayerCharacterSprite::new_warrior);
player_sprite!(WizardSprite, PlayerCharacterSprite::new_wizard);
player_sprite!(RogueSprite, PlayerCharacterSprite::new_rogue);
player_sprite!(RangerSprite, PlayerCharacterSprite::new_ranger);
