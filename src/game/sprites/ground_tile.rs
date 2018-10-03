use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::map::tile_type::*;
use game::sprites::render_on;
use game::sprites::RenderPosition;
use game::sprites::Sprite;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::rc::Rc;

pub struct GroundTile<'a> {
    pub texture: Rc<Texture<'a>>,
    pub source: Rect,
    pub dest: Rect,
    pub tile_size: i32,
    pub render_width: i32,
}

impl<'a> GroundTile<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let texture = main_renderer
            .texture_manager
            .load("./assets/textures/map-tiles.png")
            .unwrap();

        GroundTile {
            texture,
            source: Rect::new(0, 0, 16, 16),
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
            TileType::GrassSmallPlant1 => {
                self.source.set_x(6 * self.tile_size);
                self.source.set_y(1 * self.tile_size);
            }
            TileType::GrassSmallPlant2 => {
                self.source.set_x(6 * self.tile_size);
                self.source.set_y(2 * self.tile_size);
            }
            TileType::GrassSmallPlant3 => {
                self.source.set_x(6 * self.tile_size);
                self.source.set_y(3 * self.tile_size);
            }
            TileType::GrassSmallPlant4 => {
                self.source.set_x(6 * self.tile_size);
                self.source.set_y(4 * self.tile_size);
            }
            TileType::GrassSmallPlant5 => {
                self.source.set_x(6 * self.tile_size);
                self.source.set_y(5 * self.tile_size);
            }
            TileType::GrassSmallRock => {
                self.source.set_x(6 * self.tile_size);
                self.source.set_y(6 * self.tile_size);
            }
            TileType::Grass => {
                self.source.set_x(6 * self.tile_size);
                self.source.set_y(6 * self.tile_size);
            }
            // Pond
            TileType::PondLeftTop => {
                self.source.set_x(0 * self.tile_size);
                self.source.set_y(6 * self.tile_size);
            }
            TileType::PondLeft => {
                self.source.set_x(0 * self.tile_size);
                self.source.set_y(7 * self.tile_size);
            }
            TileType::PondLeftBottom => {
                self.source.set_x(0 * self.tile_size);
                self.source.set_y(11 * self.tile_size);
            }
            TileType::PondBottom => {
                self.source.set_x(1 * self.tile_size);
                self.source.set_y(11 * self.tile_size);
            }
            TileType::PondRightBottom => {
                self.source.set_x(5 * self.tile_size);
                self.source.set_y(11 * self.tile_size);
            }
            TileType::PondRight => {
                self.source.set_x(5 * self.tile_size);
                self.source.set_y(10 * self.tile_size);
            }
            TileType::PondRightTop => {
                self.source.set_x(5 * self.tile_size);
                self.source.set_y(6 * self.tile_size);
            }
            TileType::PondTop => {
                self.source.set_x(1 * self.tile_size);
                self.source.set_y(6 * self.tile_size);
            }
            TileType::PondWater => {
                self.source.set_x(3 * self.tile_size);
                self.source.set_y(7 * self.tile_size);
            }
            TileType::TinyGreenBush => {
                self.source.set_x(17 * self.tile_size);
                self.source.set_y(11 * self.tile_size);
            }

            _ => {
                self.source.set_x(7 * self.tile_size);
                self.source.set_y(1 * self.tile_size);
            }
        }
    }
}

impl<'a> RenderPosition for GroundTile<'a> {
    fn render_on(&mut self, x: &usize, y: &usize) {
        render_on(&mut self.dest, &(self.render_width as usize), x, y);
    }
}

impl<'a> Sprite<'a> for GroundTile<'a> {
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
