use std::io::Write;

use crate::utils::database::{Database, Records};

pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

    pub fn run(&self) -> anyhow::Result<()> {
        self.render_main_menu();
        let choice = self.get_user_input()?;

        match choice.as_str() {
            "" => self.run_game(),
            _ => {
                println!("Unknown option: {}", choice);
                Ok(())
            }
        }
    }

    fn render_main_menu(&self) {
        // TODO make UI nice-looking all over the place, use colors, etc.
        println!("====================================");
        println!("        Welcome to Phrasey     ");
        println!("  Your command-line phrase trainer  ");
        println!("====================================\n\n");
        println!("  Press Enter to start...");
    }

    fn get_user_input(&self) -> anyhow::Result<String> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_lowercase().to_string())
    }

    fn run_game(&self) -> anyhow::Result<()> {
        // TODO read db path from config
        let db = Database::new("db.csv")?;
        // TODO read limit from config
        let limit = 1;

        match self.game_loop(&db, limit) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error during game loop: {}", e);
                Err(e)
            }
        }
    }

    fn game_loop(&self, db: &Database, limit: usize) -> anyhow::Result<()> {
        // TODO add exit option (shortcut, configurable)
        loop {
            self.start_round(db.get_random(Some(limit)))?;

            print!("Round completed! Do you want to play again? ([Y]es/no): ");
            std::io::stdout().flush()?;

            let play_again = self.get_user_input()?;
            if !["y", "yes"].contains(&play_again.as_str()) {
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
