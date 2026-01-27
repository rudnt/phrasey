use std::io::Write;

use crate::utils::database::{Database, Records};

pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

    pub fn main_loop(&self) -> anyhow::Result<()> {
        // TODO read db path from config
        let db = Database::new("db.csv")?;
        // TODO read limit from config
        let limit = 1;

        // TODO make UI nice-looking all over the place, use colors, etc.
        println!("Hi there! It's Phrasey! Let's practice!\n");
        // TODO add exit option (shortcut, configurable)
        loop {
            self.start_round(db.get_random(Some(limit)))?;

            print!("Round completed! Do you want to play again? ([Y]es/no): ");
            std::io::stdout().flush()?;

            let mut play_again = String::new();
            std::io::stdin().read_line(&mut play_again)?;

            if !["y", "yes"].contains(&play_again.trim().to_lowercase().as_str()) {
                break;
            }
        }

        Ok(())
    }

    fn start_round(&self, mut sentences: Records) -> anyhow::Result<()> {
        println!("New round! Translate the following sentences:\n");

        let mut current: usize = 0;
        while !sentences.is_empty() {
            // Clear the screen below New round line
            let (original, translation) = &sentences[current];

            println!("Sentence: {}", original);
            print!("Your translation: ");
            std::io::stdout().flush()?;

            let mut answer = String::new();
            std::io::stdin().read_line(&mut answer)?;

            if answer.trim().to_lowercase() == translation.trim().to_lowercase() {
                println!("Correct!\n");
                sentences.remove(current);
            } else {
                println!("Incorrect! The correct translation is: {}\n", translation);
                current += 1;
            }

            current %= sentences.len().max(1);
        }

        Ok(())
    }
}
