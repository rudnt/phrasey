use rand::seq::SliceRandom;
use std::fs::File;

pub type OriginalSentence = String;
pub type Translation = String;
pub type Records = Vec<(OriginalSentence, Translation)>;

pub struct Database {
    records: Records,
}

impl Database {
    pub fn new(filepath: &str) -> anyhow::Result<Self> {
        // TODO use SQLite, divide per language, include metadata, etc.
        let records = Database::from_csv(filepath)?;
        Ok(Database { records })
    }

    pub fn get_random(&self, limit: Option<usize>) -> Records {
        let mut records = self.records.clone();
        // TODO check randomness of this solution
        let mut random_generator = rand::rng();
        records.shuffle(&mut random_generator);

        match limit {
            Some(n) => records.iter().take(n).cloned().collect(),
            None => records,
        }
    }

    // TODO use it as tool to read new data into DB
    fn from_csv(path: &str) -> anyhow::Result<Records> {
        let mut records = Vec::new();
        let file = File::open(path)?;
        let mut reader = csv::ReaderBuilder::new().flexible(true).from_reader(file);

        for result in reader.records() {
            let record = result?;
            if record.len() == 2 {
                let key = record[0].to_string();
                let value = record[1].to_string();
                records.push((key, value));
                log::debug!("Row added: {:?}", record);
            } else {
                log::warn!("Row skipped: {:?}", record);
            }
        }

        Ok(records)
    }
}
