use std::io::Write;

mod utils;
use utils::database::Database;

fn main() -> anyhow::Result<()> {
    // TODO read db path from config
    let db = Database::new("db.csv")?;
    // TODO read limit from config
    let increment = 5;
    let mut limit = increment;
    let mut offset = 0;
    
    // TODO make UI nice-looking all over the place, use colors, etc.
    println!("Hi there! It's Phrasey! Let's practice!\n");
    // TODO add exit option (shortcut, configurable)
    loop {
        let mut sentences = db.get_random(Some(limit), Some(offset));
        let mut current: usize = 0;

        println!("New round! Translate the following sentences:\n");
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

            current = current % sentences.len().max(1);
        }

        print!("Round completed! Do you want to play again? (yes/no): ");
        std::io::stdout().flush()?;

        let mut play_again = String::new();
        std::io::stdin().read_line(&mut play_again)?;

        // TODO correct is y/yes
        if play_again.trim().to_lowercase() == "yes"{
            offset += limit;
            limit += increment;
        } else {
            break;
        }
    }

    Ok(())
}
