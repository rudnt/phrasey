use anyhow::Context;
use log::{debug, trace};
use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::config::Config;
use crate::utils::database::{Database, Phrase};

/// Main heart of the application that controls the whole game state.
///
/// The `Engine` manages the flow of a phrase learning game, including:
/// - Loading phrases from the database
/// - Tracking which phrases have been recognized/guessed correctly
/// - Tracking attempts for unrecognized phrases
/// - Managing the current phrase iteration
/// - Persisting results back to the database
pub struct Engine {
    config: Rc<RefCell<Config>>,
    db: Database,
    unrecognized_phrases: Vec<(Phrase, usize)>, // TODO do struct with metadata instead of tuple?
    recognized_phrases: Vec<(Phrase, usize)>,   // TODO do struct with metadata instead of tuple?
    current_phrase_idx: Option<usize>,
}

impl Engine {
    /// Creates a new Engine instance and sets up connection with the database.
    ///
    /// # Arguments
    ///
    /// * `config` - Reference-counted configuration object containing database connection details
    ///
    /// # Returns
    ///
    /// * `Ok(Engine)` - Successfully initialized engine with database connection
    /// * `Err` - If database connection fails
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

    /// Fetches phrases for a new round from the database.
    ///
    /// Retrieves a set number of phrases (configured in `phrases_per_round`) and initializes
    /// the game state for a new round. All phrases start as unrecognized with 0 attempts.
    /// The current phrase index is set to the first phrase.
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

    /// Allows iteration over remaining phrases that the user didn't guess yet.
    ///
    /// Returns the current phrase in the iteration sequence. Only unrecognized phrases
    /// are included in the iteration.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(&Phrase))` - The current phrase to be guessed
    /// * `Ok(None)` - No more phrases available in this round
    /// * `Err` - If current game state is invalid (e.g., no current phrase index set)
    pub fn get_phrase(&mut self) -> anyhow::Result<Option<&Phrase>> {
        if self.unrecognized_phrases.is_empty() {
            trace!("No more phrases available for this round");
            return Ok(None);
        }

        let index = self
            .current_phrase_idx
            .context("No current phrase index set")?;
        let phrase = &self.unrecognized_phrases[index].0;
        trace!(
            "Phrase {:?} fetched (idx: {}, len: {})",
            phrase,
            index,
            self.unrecognized_phrases.len()
        );
        Ok(Some(phrase))
    }

    /// Checks the correctness of the answer against the current phrase's translation.
    ///
    /// Compares the user's answer with the expected translation (case-insensitive,
    /// whitespace-trimmed).
    ///
    /// # Arguments
    ///
    /// * `answer` - The user's translation attempt
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Answer matches the expected translation
    /// * `Ok(false)` - Answer does not match
    /// * `Err` - If current game state is invalid (e.g., no current phrase index set)
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

    /// Moves the iteration to the next phrase.
    ///
    /// If the phrase was guessed correctly, it's moved from unrecognized to recognized phrases.
    /// If not guessed, the attempt counter is incremented and the phrase remains in the
    /// unrecognized pool. The iteration then advances to the next unrecognized phrase.
    ///
    /// # Arguments
    ///
    /// * `guessed` - `true` if the user provided the correct answer, `false` otherwise
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully advanced to next phrase
    /// * `Err` - If current game state is invalid (e.g., no current phrase index set)
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

        self.current_phrase_idx = if !self.unrecognized_phrases.is_empty() {
            Some((index + 1) % self.unrecognized_phrases.len())
        } else {
            None
        };

        Ok(())
    }

    /// Clears the game state and updates the database with round results.
    ///
    /// Resets all internal state including recognized and unrecognized phrases,
    /// and the current phrase index. This prepares the engine for a new round.
    ///
    /// # Note
    ///
    /// Database update with results is planned but not yet implemented.
    pub fn end_round(&mut self) {
        trace!("Ending round, clearing phrases");
        // TODO update DB with results before clearing phrases
        self.unrecognized_phrases.clear();
        self.recognized_phrases.clear();
        self.current_phrase_idx = None;
        debug!("Round ended, phrases cleared");
    }
}
