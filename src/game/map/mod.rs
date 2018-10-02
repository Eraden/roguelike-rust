pub mod tile_type;
pub mod layer_type;
pub mod layer;
pub mod map;

use std::fs::read_to_string;

use game::map::map::*;

#[derive(Debug)]
pub struct VisibleRange {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub map_width: u32,
    pub map_height: u32,
}

impl VisibleRange {
    pub fn new(x: i32, y: i32, width: u32, height: u32, map_width: u32, map_height: u32) -> Self {
        VisibleRange {
            x,
            y,
            width,
            height,
            map_width,
            map_height,
        }
    }
}

pub fn load_map(name: &String) -> Map {
    let path = format!("./assets/maps/{}.map", name);
    read_to_string(&path)
        .expect(&format!("To find map in {:?} there is no such file", path))
        .parse::<Map>()
        .unwrap()
}
