use log::trace;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use crate::config::Config;

pub enum Screen {
    MainMenu,
    SettingsMenu,
    GuessingScreen { original: String },
}

pub struct Renderer {
    config: Rc<RefCell<Config>>,
}

impl Renderer {
    pub fn new(config: Rc<RefCell<Config>>) -> Self {
        Renderer { config }
    }

    pub fn render(&self, screen: Screen, user_input: Option<&str>) -> anyhow::Result<()> {
        match screen {
            Screen::MainMenu => self.render_main_menu(user_input),
            Screen::SettingsMenu => self.render_settings_menu(user_input),
            Screen::GuessingScreen { original } => self.render_guessing_screen(&original, user_input),
        }
    }

    fn render_main_menu(&self, user_input: Option<&str>) -> anyhow::Result<()> {
        // TODO consider using crossterm to clear terminal and manipulate its content (for compatibility reasons)
        // TODO let's find size of the terminal and render UI nicely at the top centered
        // TODO Let's add some colors to the menu (something CyberPunk-themed)
        self.clear_screen();
        self.render_logo();
        self.render_main_menu_options();
        self.render_input_box(user_input, "Enter your choice...")?;

        trace!("Main menu rendered");
        Ok(())
    }

    fn render_settings_menu(&self, user_input: Option<&str>) -> anyhow::Result<()> {
        self.clear_screen();
        self.render_logo();
        self.render_settings_options();
        self.render_input_box(user_input, "Enter your choice...")?;

        trace!("Settings menu rendered");
        Ok(())
    }

    fn render_guessing_screen(
        &self,
        original: &str,
        user_input: Option<&str>,
    ) -> anyhow::Result<()> {
        self.clear_screen();
        self.render_logo();
        self.render_original_phrase(original);
        self.render_input_box(user_input, "Enter your answer...")?;

        trace!("Game screen rendered for phrase: {}", original);
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

    fn render_settings_options(&self) {
        println!("   Settings\n");
        println!(
            "    [D] Database URI: {}",
            self.config.borrow().db_conn_string
        );
        println!(
            "    [P] Phrases per round: {}",
            self.config.borrow().phrases_per_round
        );
        println!("    [S] Save");
        println!("    [Q] Quit");
        println!();

        trace!("Settings options rendered");
    }

    fn render_original_phrase(&self, original: &str) {
        println!("   Sentence: {}\n", original);
        trace!("Original phrase rendered: {}", original);
    }
}
