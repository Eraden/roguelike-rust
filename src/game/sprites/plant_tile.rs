use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::map::tile_type::*;
use game::sprites::render_on;
use game::sprites::RenderPosition;
use game::sprites::Sprite;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::rc::Rc;

const TILE_SIZE: u32 = 32;

pub struct PlantTile<'a> {
    pub texture: Rc<Texture<'a>>,
    pub source: Rect,
    pub dest: Rect,
    pub tile_size: i32,
    pub render_width: i32,
}

/**
    Texture have 13x13 cells 32px each
*/
impl<'a> PlantTile<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let texture = main_renderer
            .texture_manager
            .load("./assets/textures/plant-tiles.png")
            .unwrap();

        PlantTile {
            texture,
            source: Rect::new(0, 0, TILE_SIZE, TILE_SIZE),
            dest: Rect::new(
                0,
                0,
                main_renderer.config.render_tile.width,
                main_renderer.config.render_tile.height,
            ),
            tile_size: 16,
            render_width: main_renderer.config.render_tile.width as i32,
        }
    }

    pub fn set_type(&mut self, tile_type: &TileType) {
        match tile_type {
            _ => {
                self.source.set_x(11 * self.tile_size);
                self.source.set_y(3 * self.tile_size);
            }
        }
    }
}

impl<'a> RenderPosition for PlantTile<'a> {
    fn render_on(&mut self, x: &usize, y: &usize) {
        render_on(&mut self.dest, &(self.render_width as usize), x, y);
    }
}

impl<'a> Sprite<'a> for PlantTile<'a> {
    fn update(&mut self, _ticks: i32) {
        ()
    }

    fn render(&mut self, canvas: &mut WindowCanvas, _main_renderer: &mut MainRenderer<'a, 'a>) {
        canvas
            .copy_ex(
                &self.texture,
                Some(self.source),
                Some(self.dest),
                0.0,
                None,
                false,
                false,
            ).unwrap();
    }
}
