use serde::{Deserialize, Serialize};

pub enum Command {
    Quit,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub enum UserInput {
    Phrase(String),
    Command(Command),
}
