mod utils;
use rand::seq::SliceRandom;
use utils::database::Database;

fn main() -> anyhow::Result<()> {
    let db = Database::new("db.csv")?;
    let mut records = db.get_records().clone();
    let mut random_generator = rand::rng();
    records.shuffle(&mut random_generator);

    println!("Let's practice with Phrasey!\n");
    for (original, translation) in records {
        println!("Sentence: {}", original);
        println!("Your translation: ");

        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer)?;
        
        if answer.trim().to_lowercase() == translation.trim().to_lowercase() {
            println!("Correct!\n");
        } else {
            println!("Incorrect! The correct translation is: {}\n", translation);
        }
    }

    Ok(())
}
