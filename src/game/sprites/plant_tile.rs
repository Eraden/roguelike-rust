use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::map::tile_type::*;
use game::sprites::*;
use sdl2::rect::Rect;

const TILE_SIZE: u32 = 32;

pub struct PlantTile<'a> {
    pub tile_size: i32,
    renderable: Renderable<'a>,
}

/**
    Texture have 13x13 cells 32px each
*/
impl<'a> PlantTile<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let config = { main_renderer.config.clone() };
        PlantTile {
            tile_size: 16,
            renderable: Renderable::new(
                main_renderer,
                "./assets/textures/plant-tiles.png",
                config.render_tile.width as i32,
                Rect::new(0, 0, TILE_SIZE, TILE_SIZE),
                Rect::new(0, 0, config.render_tile.width, config.render_tile.height),
            ),
        }
    }

    pub fn set_type(&mut self, tile_type: &TileType) {
        match tile_type {
            _ => {
                self.renderable.source.set_x(11 * self.tile_size);
                self.renderable.source.set_y(3 * self.tile_size);
            }
        }
    }
}

impl<'a> RenderPosition for PlantTile<'a> {
    fn render_on(&mut self, x: &usize, y: &usize) {
        let w = { self.renderable.dest_size };
        render_on(&mut self.renderable.dest, &(w as usize), x, y);
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

impl<'a> Sprite<'a> for PlantTile<'a> {
    fn update(&mut self, _ticks: i32) {
        ()
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.renderable.render(canvas, main_renderer)
    }
}
