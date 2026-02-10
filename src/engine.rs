use anyhow::Context;
use log::{debug, trace};
use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::config::Config;
use crate::utils::database::{Database, Phrase};

pub struct Engine {
    config: Rc<RefCell<Config>>,
    db: Database,
    unrecognized_phrases: Vec<(Phrase, usize)>, // TODO do struct with metadata instead of tuple?
    recognized_phrases: Vec<(Phrase, usize)>,   // TODO do struct with metadata instead of tuple?
    current_phrase_idx: Option<usize>,
}

impl Engine {
    pub fn new(config: Rc<RefCell<Config>>) -> anyhow::Result<Self> {
        trace!("Initializing engine with config: {:?}", config.borrow());

        let db = Database::new(&config.borrow().db_conn_string)?;
        let engine = Engine {
            config,
            db,
            unrecognized_phrases: Vec::new(),
            recognized_phrases: Vec::new(),
            current_phrase_idx: None,
        };

        debug!("Engine initialized");
        Ok(engine)
    }

     pub fn start_round(&mut self) {
        trace!("Starting new round, fetching phrases from database");
        let phrases = self.db.get_phrases(self.config.borrow().phrases_per_round);
        self.unrecognized_phrases = phrases.into_iter().map(|phrase| (phrase, 0)).collect();
        self.current_phrase_idx = Some(0);
        debug!(
            "Round started with {} phrases",
            self.unrecognized_phrases.len()
        );
    }

    pub fn get_phrase(&mut self) -> anyhow::Result<Option<&Phrase>> {
        if self.unrecognized_phrases.is_empty() {
            trace!("No more phrases available for this round");
            return Ok(None);
        }

        let index = self.current_phrase_idx.context("No current phrase index set")?;
        let phrase = &self.unrecognized_phrases[index].0;
        trace!("Phrase {:?} fetched (idx: {}, len: {})", phrase, index, self.unrecognized_phrases.len());
        Ok(Some(phrase))
    }

    pub fn check_phrase(&mut self, answer: &String) -> anyhow::Result<bool> {
        let index = self
            .current_phrase_idx
            .context("No current phrase index set")?;
        let expected = &self.unrecognized_phrases[index].0.1;

        // TODO implement validation logic, e.g. using Levenshtein distance
        let result = answer.trim().to_lowercase() == expected.trim().to_lowercase();
        trace!(
            "Check: answer: '{}', expected: '{}', result: {}",
            answer, expected, result
        );
        Ok(result)
    }

    pub fn advance_phrase(&mut self, guessed: bool) -> anyhow::Result<()> {
        let index = self
            .current_phrase_idx
            .context("No current phrase index set")?;

        if guessed {
            let (phrase, attempts) = self.unrecognized_phrases.remove(index);
            self.recognized_phrases.push((phrase, attempts));
        } else {
            self.unrecognized_phrases[index].1 += 1;
        }
        
        self.current_phrase_idx = if !self.unrecognized_phrases.is_empty() {Some((index + 1) % self.unrecognized_phrases.len())} else {None};
        
        Ok(())
    }

    pub fn end_round(&mut self) {
        trace!("Ending round, clearing phrases");
        // TODO update DB with results before clearing phrases
        self.unrecognized_phrases.clear();
        self.recognized_phrases.clear();
        self.current_phrase_idx = None;
        debug!("Round ended, phrases cleared");
    }
}
