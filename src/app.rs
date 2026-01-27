use std::io::Write;

use crate::config::Config;
use crate::utils::database::{Database, Records};

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        App { config }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        loop {
            self.render_main_menu();
            let choice = self.get_input("Your choice: ")?;

            match choice.as_str() {
                "" => {
                    if self.run_game()? {
                        break Ok(());
                    }
                }
                // TODO add options to change settings, view & edit database, etc.
                "q" | "quit" => {
                    println!("\nGoodbye!");
                    break Ok(());
                }
                _ => {
                    println!("\nWhat did you mean by '{}'? Please try again.", choice);
                    break self.run();
                }
            }
        }
    }

    fn render_main_menu(&self) {
        // TODO make UI nice-looking all over the place, use colors, etc.
        println!("\n\n====================================");
        println!("        Welcome to Phrasey     ");
        println!("  Your command-line phrase trainer  ");
        println!("====================================\n\n");
        println!("What do you want to do?\n");
        println!("[Enter] Start a new game");
        println!("[q]     Quit\n");
    }

    fn get_input(&self, msg: &str) -> anyhow::Result<String> {
        print!("{}", msg);
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_lowercase().to_string())
    }

    /// Runs the main game logic, managing the game loop and player interactions.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the player chooses to quit after a round, `Ok(false)` if they choose to play again,
    /// or an `Err` if an error occurs during the game loop.
    fn run_game(&self) -> anyhow::Result<bool> {
        // TODO read db path from config
        let db = Database::new(&self.config.database_path)?;
        // TODO read limit from config

        match self.game_loop(&db, self.config.phrases_per_round) {
            Ok(x) => Ok(x),
            Err(e) => {
                eprintln!("Error during game loop: {}", e);
                Err(e)
            }
        }
    }

    fn game_loop(&self, db: &Database, phrases_per_round: usize) -> anyhow::Result<bool> {
        // TODO add exit option (shortcut, configurable)
        loop {
            self.start_round(db.get_random(Some(phrases_per_round)))?;

            let msg = "Round completed! Do you want to play again? (yes/no/quit): ";
            let choice = self.get_input(msg)?;

            match choice.as_str() {
                "y" | "yes" => (),
                "q" | "quit" => break Ok(true),
                _ => break Ok(false),
            }
        }
    }

    fn start_round(&self, mut sentences: Records) -> anyhow::Result<()> {
        println!("\nNew round! Translate the following sentences:\n");

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
