use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::sprites::map_sprite_tile::*;
use game::sprites::Sprite;

pub struct MapSpriteLayer<'a> {
    pub sprites: Vec<MapSpriteTile<'a>>,
}

impl<'a> MapSpriteLayer<'a> {
    pub fn new() -> Self {
        MapSpriteLayer {
            sprites: Vec::new(),
        }
    }
}

impl<'a> Sprite<'a> for MapSpriteLayer<'a> {
    fn update(&mut self, ticks: i32) {
        for i in 0..self.sprites.len() {
            self.sprites[i].update(ticks);
        }
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        for i in 0..self.sprites.len() {
            self.sprites[i].render(canvas, main_renderer);
        }
    }
}
