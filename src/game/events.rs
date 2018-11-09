#[derive(Debug)]
pub enum UpdateResult {
    NoOp,
    Stop,
    StartFirstMap,
    PickCharacter,
}
