use anyhow::Context;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use log::{debug, error, info};
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use std::time::Duration;

use crate::config::Config;
use crate::engine::Engine;
use crate::types::{Command, UserInput};

pub struct App {
    config: Rc<RefCell<Config>>,
    engine: Engine,
}

impl App {
    pub fn new(config: Rc<RefCell<Config>>) -> anyhow::Result<Self> {
        let engine = Engine::new(config.clone()).context("Failed to initialize engine")?;
        Ok(App { config, engine })
    }

    // TODO better handle cases where msg size is bigger than input box width
    // TODO render Phrasey logo in the top of the screen and change text beneath it to match the current menu (main menu, settings, game, etc.)
    // don't scroll the screen, just update the text in place
    pub fn run(&mut self) -> anyhow::Result<()> {
        self.render_logo();

        let mut input_box_text = "Choose and press Enter";
        loop {
            println!();
            println!("  What do you want to do?\n");
            println!("   [Enter]  New game");
            println!("   [S]      Settings");
            println!("   [Q]      Quit\n");

            let choice = self.get_input(input_box_text)?;

            match choice {
                UserInput::Command(cmd) => match cmd {
                    Command::Quit => {
                        debug!("User triggered quit shortcut in main menu");
                        println!("\nGoodbye!\n");
                        break Ok(());
                    }
                },
                UserInput::Phrase(phrase) => match phrase.as_str() {
                    "" => {
                        debug!("User chose to start a new game");
                        if self.run_game()? {
                            debug!("User chose to quit after game");
                            break Ok(());
                        }
                    }
                    "s" | "settings" => {
                        debug!("User chose settings with {}", phrase);
                        if self.run_settings()? {
                            debug!("User chose to quit after settings");
                            break Ok(());
                        }
                    }
                    // TODO add options to view & edit database, etc.
                    "q" | "quit" => {
                        debug!("User chose to quit the application with {}", phrase);
                        println!("\nGoodbye!\n");
                        break Ok(());
                    }
                    _ => {
                        debug!("Unrecognized input in main menu: {}", phrase);
                        print!("\x1b[3A");
                        input_box_text = "Unrecognized option. Choose an option and press Enter";
                    }
                },
            }
        }
    }

    fn render_logo(&self) {
        // TODO let's find size of the terminal, clear it and render UI nicely at the top
        // TODO Let's add some colors to the menu (something CyberPunk-themed)
        println!();
        println!("  ██████╗ ██╗  ██╗██████╗  █████╗ ███████╗███████╗██╗   ██╗");
        println!("  ██╔══██╗██║  ██║██╔══██╗██╔══██╗██╔════╝██╔════╝╚██╗ ██╔╝");
        println!("  ██████╔╝███████║██████╔╝███████║███████╗█████╗   ╚████╔╝ ");
        println!("  ██╔═══╝ ██╔══██║██╔══██╗██╔══██║╚════██║██╔══╝    ╚██╔╝  ");
        println!("  ██║     ██║  ██║██║  ██║██║  ██║███████║███████╗   ██║   ");
        println!("  ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝   ╚═╝   ");
        debug!("Logo rendered");
    }

    fn get_input(&self, msg: &str) -> anyhow::Result<UserInput> {
        let box_width = self.config.borrow().input_box_width;
        let top_border = format!("┌{}┐", "─".repeat(box_width));
        let text_line = format!(
            "│ \x1b[90m{}\x1b[0m {}│",
            msg,
            " ".repeat(box_width - msg.len() - 2)
        );
        let bottom_border = format!("└{}┘", "─".repeat(box_width));

        println!("{}", top_border);
        println!("{}", text_line);
        println!("{}", bottom_border);
        print!("\x1b[2A\x1b[2C");
        std::io::stdout().flush()?;

        enable_raw_mode()?;
        let mut input = String::new();

        loop {
            if event::poll(Duration::from_millis(100))?
                && let Event::Key(KeyEvent {
                    code, modifiers, ..
                }) = event::read()?
            {
                if (modifiers.contains(KeyModifiers::CONTROL)) && code == KeyCode::Char('d') {
                    debug!("User triggered quit shortcut during input");
                    disable_raw_mode()?;
                    println!("\n{}", bottom_border);
                    return Ok(UserInput::Command(Command::Quit));
                }

                match code {
                    KeyCode::Enter => {
                        debug!("User finished input: {}", input);
                        disable_raw_mode()?;
                        println!("\n{}", bottom_border);
                        return Ok(UserInput::Phrase(input.trim().to_lowercase()));
                    }
                    KeyCode::Char(c) => {
                        if input.is_empty() {
                            print!("{}|", " ".repeat(box_width - 1));
                            print!("\x1b[{}D", box_width);
                        } else if input.len().is_multiple_of(box_width - 2) {
                            print!("\n\x1b[{}D", box_width);
                            println!("|{}|", " ".repeat(box_width));
                            print!("\x1b[{}D", box_width + 2);
                            println!("{}", bottom_border);
                            print!("\x1b[2A\x1b[{}D", box_width);
                            std::io::stdout().flush()?;
                        }
                        input.push(c);
                        print!("{}", c);
                        std::io::stdout().flush()?;
                    }
                    KeyCode::Backspace => {
                        if input.is_empty() {
                            continue;
                        } else if input.len() == 1 {
                            input.pop();
                            print!("\x1b[{}D", input.len() + 4);
                            print!("{}", text_line);
                            print!("\x1b[{}D", box_width);
                            std::io::stdout().flush()?;
                        } else if input.len().is_multiple_of(box_width - 2) {
                            input.pop();
                            print!("\x1b[2D");
                            print!("\n{}", " ".repeat(box_width + 4));
                            print!("\x1b[{}D\x1b[1A", box_width + 4);
                            print!("{}", bottom_border);
                            print!("\x1b[A\x1b[2D");
                            print!("\x08 \x08");
                            std::io::stdout().flush()?;
                        } else {
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

    /// Runs the main game logic, managing the game loop and player interactions.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the player chooses to quit after a round, `Ok(false)` if they choose to play again,
    /// or an `Err` if an error occurs during the game loop.
    fn run_game(&mut self) -> anyhow::Result<bool> {
        self.engine.start_round();

        match self.game_loop() {
            Ok(x) => Ok(x),
            Err(e) => {
                eprintln!("Error during game loop: {}", e);
                error!("Error during game loop: {}", e);
                Err(e)
            }
        }
        // TODO think about a better way to handle quiting
    }

    fn game_loop(&mut self) -> anyhow::Result<bool> {
        info!("Game loop started.");
        loop {
            if self.start_round()? {
                debug!("User chose to quit during round");
                break Ok(true);
            }

            let msg = "Do you want to play again? (yes/no/quit): ";
            let choice = self.get_input(msg)?;

            match choice {
                UserInput::Command(cmd) => match cmd {
                    Command::Quit => {
                        debug!("User triggered quit shortcut after round");
                        println!("\nGoodbye!\n");
                        break Ok(true);
                    }
                },
                UserInput::Phrase(choice) => match choice.as_str() {
                    "y" | "yes" => {
                        debug!("User chose to play another round with {}", choice);
                    }
                    "q" | "quit" => {
                        debug!("User chose to quit the game with {}", choice);
                        println!("\nGoodbye!\n");
                        break Ok(true);
                    }
                    _ => {
                        debug!("User chose to come back to main menu with {}", choice);
                        break Ok(false);
                    }
                },
            }
        }
    }

    fn start_round(&mut self) -> anyhow::Result<bool> {
        println!("\nNew round! Translate the following sentences:\n");
        debug!("Starting a new round");

        while let Some(phrase) = self.engine.get_next_phrase() {
            // Clear the screen below New round line
            let (original, translation) = phrase.clone();

            println!("Sentence: {}\n", original);
            let answer = self.get_input("Your translation: ")?;

            if let UserInput::Command(cmd) = &answer
                && matches!(cmd, Command::Quit)
            {
                debug!("User triggered quit shortcut during round");
                println!("\nGoodbye!\n");
                return Ok(true);
            } else if let UserInput::Phrase(phrase) = &answer
                && self.engine.check_current_phrase_and_move_on(phrase)?
            {
                println!("\nCorrect!\n");
                debug!(
                    "Correct answer: original = '{}', translation = '{}'",
                    original, translation
                );
            } else {
                println!("\nWrong! The correct translation is: {}\n", translation);
                debug!(
                    "Wrong answer: original = '{}', translation = '{}'",
                    original, translation
                );
            }
        }

        debug!("Round completed");
        Ok(false)
    }

    fn run_settings(&mut self) -> anyhow::Result<bool> {
        // TODO let's find size of the terminal, clear it and render UI nicely at the top
        // TODO Let's add some colors to the menu (something CyberPunk-themed)
        // TODO adjust settings to fit nicely with other parts of the UI
        let mut new_phrases_per_round = None;
        loop {
            println!("\nSettings menu\n");
            println!("[d] Database URI: {}", self.config.borrow().db_conn_string);
            println!(
                "[p] Phrases per round: {}",
                self.config.borrow().phrases_per_round
            );
            println!("[s] Save\n");
            println!("[q] Quit\n");

            let choice = self.get_input("Your choice: ")?;
            match choice {
                UserInput::Command(cmd) => match cmd {
                    Command::Quit => {
                        debug!("User triggered quit shortcut during settings menu");
                        println!("\nGoodbye!\n");
                        break Ok(true);
                    }
                },
                UserInput::Phrase(option) => match option.as_str() {
                    "p" | "phrases" => {
                        debug!("User chose to change number of phrases per round");
                        let new_limit =
                            self.get_input("Enter new number of phrases per round: ")?;
                        match new_limit {
                            UserInput::Command(cmd) => match cmd {
                                Command::Quit => {
                                    debug!(
                                        "User triggered quit shortcut during phrases per round input"
                                    );
                                    println!("\nGoodbye!\n");
                                    return Ok(true);
                                }
                            },
                            UserInput::Phrase(limit) => {
                                new_phrases_per_round = Some(limit.parse::<usize>()?);
                                println!("Number of phrases changed.");
                                info!(
                                    "User provided new number of phrases per round: '{}' to '{}'",
                                    self.config.borrow().phrases_per_round,
                                    new_phrases_per_round
                                        .context("Failed to parse new phrases per round limit")?
                                );
                            }
                        }
                    }
                    "s" | "save" => {
                        debug!("User chose to save settings");
                        if let Some(p) = new_phrases_per_round {
                            self.config.borrow_mut().phrases_per_round = p;
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
                    _ => {
                        debug!("Unrecognized input in settings menu");
                        println!("\nUnrecognized option.\n");
                    }
                },
            }
        }
    }
}
