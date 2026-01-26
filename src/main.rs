mod utils;
use utils::database::Database;

fn main() -> anyhow::Result<()> {
    let db = Database::new("db.csv")?;
    let mut offset = 0;
    let mut limit = 5;
    
    println!("Hi there! It's Phrasey! Let's practice!\n");
    loop {
        let mut sentences = db.get_random(Some(limit), Some(offset)).clone();
        let mut current: usize = 0;

        println!("New round! Translate the following sentences:\n");
        while !sentences.is_empty() {
            let (original, translation) = &sentences[current];

            println!("Sentence: {}", original);
            println!("Your translation:");
    
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

        println!("Round completed! Do you want to play again? (yes/no): ");
        let mut play_again = String::new();
        std::io::stdin().read_line(&mut play_again)?;

        if play_again.trim().to_lowercase() == "yes"{
            offset += limit;
            limit += 5;
        } else {
            break;
        }
    }

    Ok(())
}
