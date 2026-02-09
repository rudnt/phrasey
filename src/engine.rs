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
    next_phrase_idx: usize,
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
            next_phrase_idx: 0,
            current_phrase_idx: None,
        };

        debug!("Engine initialized");
        Ok(engine)
    }

    // TODO on round end we have to update DB and so on - think how to tie it together
    // So game won't be in bad state (e.g. app crash) and we won't lose progress
    // maybe we return iterable game object with db update on decostruction?
    // Think about it in context of CLI AND API - we want to be able to use it in both contexts
    pub fn start_round(&mut self) {
        trace!("Starting new round, fetching phrases from database");
        let phrases = self.db.get_phrases(self.config.borrow().phrases_per_round);
        self.unrecognized_phrases = phrases.into_iter().map(|phrase| (phrase, 0)).collect();
        self.recognized_phrases.clear();
        self.next_phrase_idx = 0;
        self.current_phrase_idx = None;
        debug!(
            "Round started with {} phrases",
            self.unrecognized_phrases.len()
        );
    }

    pub fn get_next_phrase(&mut self) -> Option<&Phrase> {
        if self.unrecognized_phrases.is_empty() {
            trace!("No more phrases available for this round");
            return None;
        }

        let index = self.next_phrase_idx % self.unrecognized_phrases.len();
        let phrase = &self.unrecognized_phrases[index].0;
        trace!(
            "Next phrase fetched {:?}, index: {}",
            phrase, self.next_phrase_idx
        );

        self.next_phrase_idx = index + 1;
        self.current_phrase_idx = Some(index);
        Some(phrase)
    }

    pub fn check_current_phrase_and_move_on(&mut self, answer: &String) -> anyhow::Result<bool> {
        let index = self
            .current_phrase_idx
            .context("No current phrase index set")?;
        let phrase = &self.unrecognized_phrases[index].0.1;

        // TODO implement validation logic, e.g. using Levenshtein distance
        let result = answer.trim().to_lowercase() == phrase.trim().to_lowercase();
        trace!(
            "Checking answer: '{}', expected: '{}', result: {}",
            answer, phrase, result
        );

        if result {
            let (phrase, attempts) = self.unrecognized_phrases.remove(index);
            self.recognized_phrases.push((phrase, attempts));
        } else {
            self.unrecognized_phrases[index].1 += 1;
        }
        self.current_phrase_idx = None;
        
        Ok(result)
    }

    // pub fn end_round();
    // pub fn end_game();
}
