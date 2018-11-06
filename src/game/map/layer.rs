use game::map::layer_type::*;
use game::map::tile_type::*;
use game::map::*;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum LayerError {
    MissingLayerMeta,
    InvalidLayerLineWidth(usize, usize),
    InvalidLayerHeight(usize, usize),
    NoMatchingLayerType,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub tiles: Vec<TileType>,
    pub layer_type: LayerType,
}

impl Layer {
    pub fn new(layer_type: &LayerType, tiles: &Vec<TileType>) -> Self {
        Self {
            layer_type: layer_type.clone(),
            tiles: tiles.clone().to_vec(),
        }
    }

    pub fn take(&self, r: &VisibleRange) -> Self {
        let mut tiles = Vec::new();
        let size = (r.width * r.height) as usize;
        tiles.reserve(size);
        (0..size)
            .clone()
            .collect::<Vec<usize>>()
            .iter()
            .for_each(|i| {
                tiles.push(
                    self.tiles[*i + (r.y as usize * r.width as usize) + r.x as usize].clone(),
                );
            });
        Layer::new(&self.layer_type, &tiles)
    }
}

impl FromStr for Layer {
    type Err = LayerError;

    fn from_str(contents: &str) -> Result<Self, <Self as FromStr>::Err> {
        let lines: Vec<&str> = contents.split("\n").collect();
        let mut it = lines.iter();

        let layer_type: LayerType = match it.next() {
            None => return Err(LayerError::MissingLayerMeta),
            Some(meta) => LayerType::as_slice()
                .iter()
                .find(|name| {
                    let s: String = name.to_string().to_lowercase();
                    let m: String = meta.to_string();
                    let res: bool = m.ends_with(&s);
                    res
                }).ok_or(LayerError::NoMatchingLayerType)?
                .clone(),
        };

        let mut a: Vec<TileType> = Vec::new();
        it.filter(|s| s.len() > 0).for_each(|line| {
            line.split::<&str>(" ")
                .collect::<Vec<&str>>()
                .iter()
                .for_each(|n| a.push(n.parse::<TileType>().unwrap()));
        });
        Ok(Layer::new(&layer_type, &a))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_stop_when_missing_metadata() {
        let content = "\n\n\n\n";
        let result: Result<Layer, LayerError> = Layer::from_str(content);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(LayerError::NoMatchingLayerType));
    }

    #[test]
    fn it_recognize_layer_type() {
        let content = "'layer ground3\n";
        let result: Result<Layer, LayerError> = Layer::from_str(content);
        assert!(result.is_ok());
        let layer = result.unwrap();
        assert_eq!(layer.layer_type, LayerType::Ground3);
    }
}
