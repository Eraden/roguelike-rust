use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::map::tile_type::*;
use game::sprites::*;
use sdl2::rect::Rect;

pub struct GroundTile<'a> {
    pub tile_size: i32,
    renderable: Renderable<'a>,
}

impl<'a> GroundTile<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let config = { main_renderer.config.clone() };
        GroundTile {
            tile_size: 16,
            renderable: Renderable::new(
                main_renderer,
                "./assets/textures/map-tiles.png",
                config.render_tile.width as i32,
                Rect::new(0, 0, 16, 16),
                Rect::new(
                    0,
                    0,
                    config.render_tile.width,
                    config.render_tile.height,
                ),
            ),
        }
    }

    pub fn set_type(&mut self, tile_type: &TileType) {
        let source = &mut self.renderable.source;
        match tile_type {
            TileType::GrassSmallPlant1 => {
                source.set_x(6 * self.tile_size);
                source.set_y(1 * self.tile_size);
            }
            TileType::GrassSmallPlant2 => {
                source.set_x(6 * self.tile_size);
                source.set_y(2 * self.tile_size);
            }
            TileType::GrassSmallPlant3 => {
                source.set_x(6 * self.tile_size);
                source.set_y(3 * self.tile_size);
            }
            TileType::GrassSmallPlant4 => {
                source.set_x(6 * self.tile_size);
                source.set_y(4 * self.tile_size);
            }
            TileType::GrassSmallPlant5 => {
                source.set_x(6 * self.tile_size);
                source.set_y(5 * self.tile_size);
            }
            TileType::GrassSmallRock => {
                source.set_x(6 * self.tile_size);
                source.set_y(6 * self.tile_size);
            }
            TileType::Grass => {
                source.set_x(6 * self.tile_size);
                source.set_y(6 * self.tile_size);
            }
            // Pond
            TileType::PondLeftTop => {
                source.set_x(0 * self.tile_size);
                source.set_y(6 * self.tile_size);
            }
            TileType::PondLeft => {
                source.set_x(0 * self.tile_size);
                source.set_y(7 * self.tile_size);
            }
            TileType::PondLeftBottom => {
                source.set_x(0 * self.tile_size);
                source.set_y(11 * self.tile_size);
            }
            TileType::PondBottom => {
                source.set_x(1 * self.tile_size);
                source.set_y(11 * self.tile_size);
            }
            TileType::PondRightBottom => {
                source.set_x(5 * self.tile_size);
                source.set_y(11 * self.tile_size);
            }
            TileType::PondRight => {
                source.set_x(5 * self.tile_size);
                source.set_y(10 * self.tile_size);
            }
            TileType::PondRightTop => {
                source.set_x(5 * self.tile_size);
                source.set_y(6 * self.tile_size);
            }
            TileType::PondTop => {
                source.set_x(1 * self.tile_size);
                source.set_y(6 * self.tile_size);
            }
            TileType::PondWater => {
                source.set_x(3 * self.tile_size);
                source.set_y(7 * self.tile_size);
            }
            TileType::TinyGreenBush => {
                source.set_x(17 * self.tile_size);
                source.set_y(11 * self.tile_size);
            }

            _ => {
                source.set_x(7 * self.tile_size);
                source.set_y(1 * self.tile_size);
            }
        }
    }
}

impl<'a> RenderPosition for GroundTile<'a> {
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

impl<'a> Sprite<'a> for GroundTile<'a> {
    fn update(&mut self, _ticks: i32) {
        ()
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.renderable.render(canvas, main_renderer)
    }
}
