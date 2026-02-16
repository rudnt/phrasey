use log::trace;
use std::cell::RefCell;
use std::rc::Rc;

use crate::config::Config;

pub struct Renderer {
    config: Rc<RefCell<Config>>,
}

impl Renderer {
    pub fn new(config: Rc<RefCell<Config>>) -> Self {
        Renderer { config }
    }

    pub fn render_main_menu(&self) {
        // TODO consider using crossterm to clear terminal and manipulate its content (for compatibility reasons)
        // TODO let's find size of the terminal and render UI nicely at the top centered
        // TODO Let's add some colors to the menu (something CyberPunk-themed)
        self.clear_screen();
        self.render_logo();
        self.render_main_menu_options();

        trace!("Main menu rendered");
    }

    pub fn render_settings_menu(&self) {
        self.clear_screen();
        self.render_logo();
        println!("  Settings");
        println!();
        println!(
            "   [D] Database URI: {}",
            self.config.borrow().db_conn_string
        );
        println!(
            "   [P] Phrases per round: {}",
            self.config.borrow().phrases_per_round
        );
        println!("   [S] Save");
        println!();
        println!("   [Q] Quit");

        trace!("Settings menu rendered");
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        trace!("Screen cleared");
    }

    fn render_logo(&self) {
        println!();
        println!("  ██████╗ ██╗  ██╗██████╗  █████╗ ███████╗███████╗██╗   ██╗");
        println!("  ██╔══██╗██║  ██║██╔══██╗██╔══██╗██╔════╝██╔════╝╚██╗ ██╔╝");
        println!("  ██████╔╝███████║██████╔╝███████║███████╗█████╗   ╚████╔╝ ");
        println!("  ██╔═══╝ ██╔══██║██╔══██╗██╔══██║╚════██║██╔══╝    ╚██╔╝  ");
        println!("  ██║     ██║  ██║██║  ██║██║  ██║███████║███████╗   ██║   ");
        println!("  ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝   ╚═╝   ");
        println!();

        trace!("Logo rendered");
    }

    fn render_main_menu_options(&self) {
        println!("  What do you want to do?\n");
        println!("   [Enter]  New game");
        println!("   [S]      Settings");
        println!("   [Q]      Quit");
        println!();

        trace!("Main menu options rendered");
    }
}
