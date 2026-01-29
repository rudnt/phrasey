use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use log::{debug, error, info};
use std::io::Write;
use std::time::Duration;

use crate::config::Config;
use crate::utils::database::{Database, Records};

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        App { config }
    }

    // TODO: bug - when we quit using shortcut during game play we are asked if we want to play again
    pub fn run(&mut self) -> anyhow::Result<()> {
        self.render_main_menu();

        let mut input_box_text = "Choose and press Enter";
        loop {
            let choice = self.get_input(input_box_text)?;

            match choice.as_str() {
                "" => {
                    debug!("User chose to start a new game");
                    if self.run_game()? {
                        debug!("User chose to quit after game");
                        break Ok(());
                    }
                }
                "s" | "settings" => {
                    debug!("User chose settings with {}", choice);
                    if self.run_settings()? {
                        debug!("User chose to quit after game");
                        break Ok(());
                    }
                }
                // TODO add options to view & edit database, etc.
                "q" | "quit" | "__!quit!__" => {
                    debug!("User chose to quit the application with {}", choice);
                    println!("\nGoodbye!\n");
                    break Ok(());
                }
                _ => {
                    debug!("Unrecognized input in main menu: {}", choice);
                    print!("\x1b[3A");
                    input_box_text = "Unrecognized option. Choose an option and press Enter";
                }
            }
        }
    }

    fn render_main_menu(&self) {
        // TODO let's find size of the terminal, clear it and render UI nicely at the top
        // TODO Let's add some colors to the menu (something CyberPunk-themed)
        println!("");
        println!("  ██████╗ ██╗  ██╗██████╗  █████╗ ███████╗███████╗██╗   ██╗");
        println!("  ██╔══██╗██║  ██║██╔══██╗██╔══██╗██╔════╝██╔════╝╚██╗ ██╔╝");
        println!("  ██████╔╝███████║██████╔╝███████║███████╗█████╗   ╚████╔╝ ");
        println!("  ██╔═══╝ ██╔══██║██╔══██╗██╔══██║╚════██║██╔══╝    ╚██╔╝  ");
        println!("  ██║     ██║  ██║██║  ██║██║  ██║███████║███████╗   ██║   ");
        println!("  ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝   ╚═╝   \n");
        println!("  What do you want to do?\n");
        println!("   [Enter]  New game");
        println!("   [s]      Settings");
        println!("   [q]      Quit\n");
        debug!("Main menu rendered");
    }

    fn get_input(&self, msg: &str) -> anyhow::Result<String> {
        let box_width = 70;
        let top_border = format!("┌{}┐", "─".repeat(box_width));
        let text_line = format!("│ \x1b[90m{}\x1b[0m {}│", msg, " ".repeat(box_width - msg.len() - 2));
        let bottom_border = format!("└{}┘", "─".repeat(box_width));
        
        // TODO render text grayed, when input provided - remove it at once, bring it back if input cleared
        println!("{}", top_border);
        println!("{}", text_line);
        println!("{}", bottom_border);
        print!("\x1b[2A\x1b[{}C", msg.len() + 2);
        std::io::stdout().flush()?;

        enable_raw_mode()?;
        let mut input = String::new();
        
        loop {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(KeyEvent { code, modifiers, ..}) = event::read()? {
                    if (modifiers.contains(KeyModifiers::CONTROL)) 
                        && code == KeyCode::Char('d') {
                        debug!("User triggered quit shortcut during input");
                        disable_raw_mode()?;
                        println!("\n{}", bottom_border);
                        return Ok("__!quit!__".to_string());
                    } 
                    
                    // TODO support multi-line input and its removal clearly
                    // box expands when new line needed, shrinks when lines removed
                    // text never goes beyond box borders (box resizes or text scrolls)
                    match code {
                        KeyCode::Enter => {
                            debug!("User finished input: {}", input);
                            disable_raw_mode()?;
                            println!("\n{}", bottom_border);
                            return Ok(input.trim().to_lowercase());
                        }
                        KeyCode::Char(c) => {
                            if input.is_empty() {
                                print!("\x1b[{}D", msg.len());
                                print!("{}|", " ".repeat(box_width - 1));
                                print!("\x1b[{}D", box_width);
                            }
                            input.push(c);
                            print!("{}", c);
                            std::io::stdout().flush()?;
                        }
                        KeyCode::Backspace => {
                            if input.len() == 1 {
                                input.pop();
                                print!("\x1b[{}D", input.len() + 3);
                                print!("{}", text_line);
                                print!("\x1b[{}D", box_width - msg.len());
                                std::io::stdout().flush()?;
                            }
                            else if !input.is_empty() {
                                input.pop();
                                print!("\x08 \x08");
                                std::io::stdout().flush()?;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Runs the main game logic, managing the game loop and player interactions.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the player chooses to quit after a round, `Ok(false)` if they choose to play again,
    /// or an `Err` if an error occurs during the game loop.
    fn run_game(&self) -> anyhow::Result<bool> {
        let db = Database::new(&self.config.database_uri)?;
        debug!("Database loaded");

        match self.game_loop(&db, self.config.phrases_per_round) {
            Ok(x) => Ok(x),
            Err(e) => {
                eprintln!("Error during game loop: {}", e);
                error!("Error during game loop: {}", e);
                Err(e)
            }
        }
        // TODO think about a better way to handle quiting
    }

    fn game_loop(&self, db: &Database, phrases_per_round: usize) -> anyhow::Result<bool> {
        info!("Game loop started.");
        // TODO add exit configurable option
        loop {
            self.start_round(db.get_random(Some(phrases_per_round)))?;

            let msg = "Round completed! Do you want to play again? (yes/no/quit): ";
            let choice = self.get_input(msg)?;

            match choice.as_str() {
                "y" | "yes" => {
                    debug!("User chose to play another round with {}", choice);
                    ()
                }
                "q" | "quit" | "__!quit!__" => {
                    debug!("User chose to quit the game with {}", choice);
                    println!("\nGoodbye!\n");
                    break Ok(true)
                }
                _ => {
                    debug!("User chose to come back to main menu with {}", choice);
                    break Ok(false)
                }
            }
        }
    }

    fn start_round(&self, mut sentences: Records) -> anyhow::Result<()> {
        println!("\nNew round! Translate the following sentences:\n");
        debug!("Starting a new round with {} sentence(s)", sentences.len());

        let mut current: usize = 0;
        while !sentences.is_empty() {
            // Clear the screen below New round line
            let (original, translation) = &sentences[current];

            println!("Sentence: {}", original);
            let answer = self.get_input("Your translation: ")?;

            if answer == "__!quit!__" {
                debug!("User triggered quit shortcut during round");
                println!("\nGoodbye!\n");
                break;
            }
            else if answer == translation.trim().to_lowercase() {
                println!("\nCorrect!\n");
                debug!("Correct answer: original = '{}', translation = '{}'", original, translation);
                sentences.remove(current);
            } else {
                println!("\nWrong! The correct translation is: {}\n", translation);
                debug!("Wrong answer: original = '{}', translation = '{}'", original, translation);
                current += 1;
            }

            current %= sentences.len().max(1);
        }

        debug!("Round completed");
        Ok(())
    }

    fn run_settings(&mut self) -> anyhow::Result<bool> {
        // TODO let's find size of the terminal, clear it and render UI nicely at the top
        // TODO Let's add some colors to the menu (something CyberPunk-themed)
        // TODO adjust settings to fit nicely with other parts of the UI
        let mut new_db = None;
        let mut new_phrases_per_round = None;
        loop {
            println!("\nSettings menu\n");
            println!("[d] Database URI: {}", self.config.database_uri);
            println!("[p] Phrases per round: {}", self.config.phrases_per_round);
            println!("[s] Save\n");
            println!("[q] Quit\n");
    
            let choice = self.get_input("Your choice: ")?;
            match choice.as_str() {
                "d" | "database" => {
                    debug!("User chose to change Database URI");
                    let new_uri = self.get_input("Enter new Database URI: ")?;
                    new_db = Some(new_uri);
                    println!("Database URI updated.");
                    info!("User changed Database URI from '{}' to '{}'", self.config.database_uri, new_db.as_ref().unwrap());
                }
                "p" | "phrases" => {
                    debug!("User chose to change number of phrases per round");
                    let new_limit = self.get_input("Enter new number of phrases per round: ")?;
                    new_phrases_per_round = Some(new_limit.parse::<usize>()?);
                    println!("Number of phrases per round updated.");
                    info!("User changed number of phrases per round from '{}' to '{}'", self.config.phrases_per_round, new_limit);
                }
                "s" | "save" => {
                    debug!("User chose to save settings");
                    match &new_db {
                        Some(db) => self.config.database_uri = db.clone(),
                        None => (),
                    }
                    match &new_phrases_per_round {
                        Some(p) => self.config.phrases_per_round = *p,
                        None => (),
                    }
                    println!("Settings saved.\n");
                    info!("Settings saved.");
                    break Ok(false);
                }
                "q" | "quit" => {
                    debug!("User chose to quit the settings menu");
                    println!("\nExiting settings menu.\n");
                    break Ok(false);
                }
                "__!quit!__" => {
                    debug!("User triggered quit shortcut during settings menu");
                    println!("\nGoodbye!\n");
                    break Ok(true);
                }
                _ => {
                    debug!("Unrecognized input in settings menu");
                    println!("\nUnrecognized option.\n");
                }
            }
        }
    }
}
