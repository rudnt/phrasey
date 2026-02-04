pub enum Command {
    Quit,
}

pub enum UserInput {
    Phrase(String),
    Command(Command),
}
