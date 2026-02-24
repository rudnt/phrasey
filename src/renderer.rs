use anyhow::Context;
use crossterm::cursor;
use crossterm::execute;
use log::trace;
use std::cell::RefCell;
use std::io::Write;
use std::io::stdout;
use std::rc::Rc;

use crate::config::Config;

pub struct Renderer {
    config: Rc<RefCell<Config>>,
}

impl Renderer {
    pub fn new(config: Rc<RefCell<Config>>) -> Self {
        Renderer { config }
    }

    pub fn render_main_menu(&self) -> anyhow::Result<()> {
        // TODO consider using crossterm to clear terminal and manipulate its content (for compatibility reasons)
        // TODO let's find size of the terminal and render UI nicely at the top centered
        // TODO Let's add some colors to the menu (something CyberPunk-themed)
        self.hide_cursor()?;
        self.clear_screen();
        self.render_logo();
        self.render_main_menu_options();

        trace!("Main menu rendered");
        Ok(())
    }

    pub fn render_settings_menu(
        &self,
        user_input: Option<&str>,
        placeholder_text: Option<&str>,
        config: &Config,
    ) -> anyhow::Result<()> {
        self.clear_screen();
        self.render_logo();
        self.render_settings_options(config);

        if placeholder_text.is_some() {
            self.render_input_box(user_input, placeholder_text.unwrap())?;
        } else if user_input.is_some() {
            self.render_input_box(user_input, "Sorry, something went wrong...")?;
        } else {
            self.hide_cursor()?;
        }

        trace!("Settings menu rendered");
        Ok(())
    }

    // TODO overall game screens should have progress indicator, some nice looking art and colors (something CyberPunk-themed)
    pub fn render_guessing_screen(
        &self,
        original: &str,
        user_input: Option<&str>,
    ) -> anyhow::Result<()> {
        self.clear_screen();
        self.render_logo();
        // TODO render proper guessing screen with some colors and maybe ASCII art (something CyberPunk-themed)
        self.render_original_phrase(original);
        self.render_input_box(user_input, "Enter your answer...")?;

        trace!("Game screen rendered for phrase: {}", original);
        Ok(())
    }

    pub fn render_feedback_screen(
        &self,
        is_correct: bool,
        correct_answer: &str,
    ) -> anyhow::Result<()> {
        self.clear_screen();
        self.render_logo();

        // TODO introduce proper feedback screen with some colors and maybe ASCII art (something CyberPunk-themed)
        if is_correct {
            println!("Correct!");
        } else {
            println!("Incorrect! The correct answer was:\n\t{}", correct_answer);
        }
        println!();

        trace!("Feedback screen rendered, is_correct={}", is_correct);
        Ok(())
    }

    pub fn render_round_end_screen(&self) -> anyhow::Result<()> {
        self.clear_screen();
        self.render_logo();
        // TODO introduce proper round end screen with some colors and maybe ASCII art (something CyberPunk-themed)
        println!("Round completed! Ready for the next one?");
        println!("    [Enter]  Next game");
        println!("    [B]      Back to main menu");
        println!();

        trace!("Round end screen rendered");
        Ok(())
    }

    pub fn render_quit_screen(&self) -> anyhow::Result<()> {
        self.hide_cursor()?;
        self.clear_screen();
        self.render_logo();
        // TODO introduce proper goodbye screen with some colors and maybe ASCII art (something CyberPunk-themed)
        println!("Goodbye!");
        println!();

        trace!("Goodbye screen rendered");
        Ok(())
    }

    fn hide_cursor(&self) -> anyhow::Result<()> {
        execute!(stdout(), cursor::Hide).context("Failed to hide cursor")?;
        trace!("Cursor hidden");
        Ok(())
    }

    fn show_cursor(&self) -> anyhow::Result<()> {
        execute!(stdout(), cursor::Show).context("Failed to show cursor")?;
        trace!("Cursor shown");
        Ok(())
    }
    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        trace!("Screen cleared");
    }

    fn render_logo(&self) {
        println!();
        println!("   ██████╗ ██╗  ██╗██████╗  █████╗ ███████╗███████╗██╗   ██╗   ");
        println!("   ██╔══██╗██║  ██║██╔══██╗██╔══██╗██╔════╝██╔════╝╚██╗ ██╔╝   ");
        println!("   ██████╔╝███████║██████╔╝███████║███████╗█████╗   ╚████╔╝    ");
        println!("   ██╔═══╝ ██╔══██║██╔══██╗██╔══██║╚════██║██╔══╝    ╚██╔╝     ");
        println!("   ██║     ██║  ██║██║  ██║██║  ██║███████║███████╗   ██║      ");
        println!("   ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝   ╚═╝      ");
        println!();

        trace!("Logo rendered");
    }

    fn render_main_menu_options(&self) {
        println!("   What do you want to do?\n");
        println!("    [Enter]  New game");
        println!("    [S]      Settings");
        println!("    [Q]      Quit");
        println!();

        trace!("Main menu options rendered");
    }

    fn render_input_box(&self, text: Option<&str>, placeholder_text: &str) -> anyhow::Result<()> {
        let box_width = self.config.borrow().input_box_width;
        let text_width = box_width - 2;

        let top_border = format!("┌{}┐", "─".repeat(box_width));
        let bottom_border = format!("└{}┘", "─".repeat(box_width));
        let text_lines = if let Some(text) = text {
            // TODO split text into chunks per line
            // TODO move cursor to the end of last character (for text)
            // TODO combine formats in a loop for multiple lines
            format!("│ {}{} │", text, " ".repeat(text_width - text.len()))
        } else {
            format!(
                "│ \x1b[90m{}\x1b[0m{} │",
                placeholder_text,
                " ".repeat(text_width - placeholder_text.len())
            )
        };

        println!(" {} ", top_border);
        println!(" {} ", text_lines);
        println!(" {} ", bottom_border);
        print!("\x1b[2A\x1b[2C");
        std::io::stdout().flush()?;

        trace!("Input box rendered");
        Ok(())
    }

    fn render_settings_options(&self, config: &Config) {
        println!("   Settings\n");
        println!(
            "    [D] Database URI: {}",
            config.db_conn_string
        );
        println!(
            "    [P] Phrases per round: {}",
            config.phrases_per_round
        );
        println!("    [S] Save");
        println!("    [B] Back to main menu");
        println!();

        trace!("Settings options rendered");
    }

    fn render_original_phrase(&self, original: &str) {
        println!("   Sentence: {}\n", original);
        trace!("Original phrase rendered: {}", original);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.show_cursor().expect("Failed to show cursor");
    }
}
