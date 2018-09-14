#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub t: bool,
}

impl<'a> Config {
    pub fn new() -> Self {
        Config {
            width: 1280,
            height: 768,
            t: false,
        }
    }
}
