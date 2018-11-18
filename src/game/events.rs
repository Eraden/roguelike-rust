use game::sprites::Gender;
use game::sprites::PlayerClass;
use game::MousePosition;

#[derive(Debug, PartialEq)]
pub enum UpdateResult {
    NoOp,
    Stop,
    StartFirstMap,
    PickCharacter,
    PlayerCharacterClicked((PlayerClass, Gender)),
    AboveButton(MousePosition),
}
