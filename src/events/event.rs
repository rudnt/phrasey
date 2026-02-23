#[derive(Debug)]
pub enum Event {
    Back,
    Enter,
    Quit,
    RemoveCharacter,
    Character(char),
}
