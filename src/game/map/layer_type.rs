#[derive(Debug, Clone, PartialEq)]
pub enum LayerType {
    Ground1,
    Ground2,
    Ground3,
    Animals,
    Plants,
    Players,
    Roofs,
}

impl LayerType {
    pub fn as_slice() -> [LayerType; 7] {
        [
            LayerType::Ground1,
            LayerType::Ground2,
            LayerType::Ground3,
            LayerType::Animals,
            LayerType::Plants,
            LayerType::Players,
            LayerType::Roofs,
        ]
    }

    pub fn to_string(&self) -> String {
        match self {
            LayerType::Ground1 => "Ground1",
            LayerType::Ground2 => "Ground2",
            LayerType::Ground3 => "Ground3",
            LayerType::Plants => "Plants",
            LayerType::Animals => "Animals",
            LayerType::Players => "Players",
            LayerType::Roofs => "Roofs",
        }
        .to_string()
    }
}
