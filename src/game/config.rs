#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderTileSize {
    pub width: u32,
    pub height: u32,
}

impl RenderTileSize {
    pub fn new(width: u32, height: u32) -> Self {
        RenderTileSize { width, height }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub render_tile: RenderTileSize,
    pub t: bool,
}

impl<'a> Config {
    pub fn new() -> Self {
        Config {
            width: 1280,
            height: 768,
            render_tile: RenderTileSize::new(32, 32),
            t: false,
        }
    }
}
