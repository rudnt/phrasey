use anyhow::Context;
use log::{debug, trace};
use rand::seq::SliceRandom;
use std::fs::File;

pub type OriginalSentence = String;
pub type Translation = String;
pub type Phrase = (OriginalSentence, Translation); // TODO change to struct
pub type Phrases = Vec<Phrase>;

pub struct Database {
    records: Phrases,
}

impl Database {
    pub fn new(conn_string: &str) -> anyhow::Result<Self> {
        // TODO use SQLite, divide per language, include metadata, etc.
        let filepath = conn_string
            .strip_prefix("file://")
            .context("Failed to parse database connection string")?;
        trace!(
            "Database connection string parsed, loading from file: {}",
            filepath
        );
        let records = Database::from_csv(filepath)?;
        debug!(
            "Database loaded from {} with {} records",
            conn_string,
            records.len()
        );
        Ok(Database { records })
    }

    pub fn get_phrases(&self, limit: usize) -> Phrases {
        let mut all_phrases = self.records.clone();
        // TODO check randomness of this solution
        let mut random_generator = rand::rng();
        all_phrases.shuffle(&mut random_generator);
        trace!("Shuffled records for randomness");

        let phrases: Phrases = all_phrases.iter().take(limit).cloned().collect();
        trace!("Fetched {} random records from database", phrases.len());
        phrases
    }

    // TODO use it as tool to read new data into DB
    fn from_csv(path: &str) -> anyhow::Result<Phrases> {
        let mut records = Vec::new();
        let file = File::open(path)?;
        let mut reader = csv::ReaderBuilder::new().flexible(true).from_reader(file);
        trace!("CSV reader initialized for file: {}", path);

        for result in reader.records() {
            let record = result?;
            if record.len() == 2 {
                let key = record[0].to_string();
                let value = record[1].to_string();
                records.push((key, value));
                trace!("Row added: {:?}", record);
            } else {
                trace!("Row skipped: {:?}", record);
            }
        }

        trace!("Total records loaded from CSV: {}", records.len());
        Ok(records)
    }
}
