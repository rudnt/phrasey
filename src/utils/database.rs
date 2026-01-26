use std::fs::File;

pub type OriginalSentence = String;
pub type Translation = String;
pub type Records = Vec<(OriginalSentence, Translation)>;

pub struct Database {
    records: Records,
}

impl Database {
    pub fn new(filepath: &str) -> anyhow::Result<Self> {
        let records = Database::from_csv(filepath)?;
        Ok(Database { records })
    }

    pub fn get_records(&self) -> Records {
        self.records.clone()
    }

    fn from_csv(path: &str) -> anyhow::Result<Records> {
        let mut records = Vec::new();
        let file = File::open(path)?;
        let mut reader = csv::ReaderBuilder::new()
            .flexible(true)
            .from_reader(file);

        for result in reader.records() {
            let record = result?;
            if record.len() == 2 {
                let key = record[0].to_string();
                let value = record[1].to_string();
                records.push((key, value));
                log::debug!("Row added: {:?}", record);
            } else {
                log::debug!("Row skipped: {:?}", record);
            }
        }

        Ok(records)
    }
}
