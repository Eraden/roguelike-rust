use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::sprites::deer_sprite::*;
use game::sprites::ground_tile::*;
use game::sprites::Sprite;

pub enum MapSpriteTileType<'a> {
    TileTypeFemaleDeer(FemaleDeerSprite<'a>),
    TileTypeMaleDeer(MaleDeerSprite<'a>),
    TileTypeGround(GroundTile<'a>),
}

pub struct MapSpriteTile<'a> {
    pub tile: MapSpriteTileType<'a>,
}

impl<'a> MapSpriteTile<'a> {
    pub fn new_female_deer_tile(female_deer: FemaleDeerSprite<'a>) -> Self {
        MapSpriteTile {
            tile: MapSpriteTileType::TileTypeFemaleDeer(female_deer),
        }
    }

    pub fn new_male_deer_tile(male_deer: MaleDeerSprite<'a>) -> Self {
        MapSpriteTile {
            tile: MapSpriteTileType::TileTypeMaleDeer(male_deer),
        }
    }

    pub fn new_ground_tile(ground: GroundTile<'a>) -> Self {
        MapSpriteTile {
            tile: MapSpriteTileType::TileTypeGround(ground),
        }
    }
}

impl<'a> Sprite<'a> for MapSpriteTile<'a> {
    fn update(&mut self, ticks: i32) {
        match self.tile {
            MapSpriteTileType::TileTypeFemaleDeer(ref mut sprite) => sprite.update(ticks),
            MapSpriteTileType::TileTypeMaleDeer(ref mut sprite) => sprite.update(ticks),
            MapSpriteTileType::TileTypeGround(ref mut sprite) => sprite.update(ticks),
        };
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        match self.tile {
            MapSpriteTileType::TileTypeFemaleDeer(ref mut sprite) => {
                sprite.render(canvas, main_renderer)
            }
            MapSpriteTileType::TileTypeMaleDeer(ref mut sprite) => {
                sprite.render(canvas, main_renderer)
            }
            MapSpriteTileType::TileTypeGround(ref mut sprite) => {
                sprite.render(canvas, main_renderer)
            }
        };
    }
}
