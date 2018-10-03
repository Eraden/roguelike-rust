use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::sprites::female_deer_sprite::*;
use game::sprites::ground_tile::*;
use game::sprites::Sprite;

pub struct MapSpriteTile<'a> {
    pub female_deer: Option<FemaleDeerSprite<'a>>,
    pub ground: Option<GroundTile<'a>>,
}

impl<'a> MapSpriteTile<'a> {
    pub fn new() -> Self {
        MapSpriteTile {
            female_deer: None,
            ground: None,
        }
    }
}

impl<'a> Sprite<'a> for MapSpriteTile<'a> {
    fn update(&mut self, ticks: i32) {
        match self.female_deer.as_mut() {
            Some(ref mut sprite) => {
                (*sprite).update(ticks);
            }
            _ => {}
        };
        match self.ground.as_mut() {
            Some(ref mut sprite) => {
                (*sprite).update(ticks);
            }
            _ => {}
        };
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        match self.female_deer.as_mut() {
            Some(ref mut sprite) => {
                (*sprite).render(canvas, main_renderer);
            }
            _ => {}
        };
        match self.ground.as_mut() {
            Some(ref mut sprite) => {
                (*sprite).render(canvas, main_renderer);
            }
            _ => {}
        };
    }
}
