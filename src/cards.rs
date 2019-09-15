use crate::fileio::Term;
use crate::jisho::{self, Japanese};
use std::fmt;

/// Character used to separate each field for a card in the card fileio
///
/// `\t` is being used to allow semi-colons to be used in cards
const SEPARATOR: char = '\t';

/// Represents an Anki flashcard.
/// Currently only a single front and back fields are supported.
pub struct Card {
    front: String,
    back: String,
}

impl Card {
    /// Generates a new card from `&str`s
    fn new(front: &str, definition: &str) -> Self {
        Card {
            front: front.into(),
            back: definition.into(),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}{}{}", self.front, SEPARATOR, self.back)
    }
}

/// Generates a card based on a search term.
/// If it's not possible to generate the card, returns `None` (or will, once
/// I implement that. Right now, failing to get a response from the API or
/// parsing JSON will simply panic)
pub fn generate_card(t: &Term) -> Option<Card> {
    match *t {
        Term::Search(ref s) => {
            println!("searching {}", s);
            search(s)
        }
        Term::Pass(ref f, ref b) => Some(Card::new(f, b)),
    }
}

/// Searches dictionary and returns appropriate card if successful
fn search(search_term: &str) -> Option<Card> {
    // Get Jisho API response
    let results = match jisho::make_request(search_term) {
        Ok(results) => results,
        Err(e) => {
            handle_error(search_term, e);
            return None;
        }
    };

    if results.is_empty() {
        return None;
    }
    // Here we have to make judgements about which result to use
    // I'll just trust that jisho's top result will be good for now
    // (which it usually is, assuming your search is also reasonable)
    let top = &results[0];
    // Now we have to construct a front of the form <kanji>[<reading>]
    // noting that either one of them might not exist, and that there may be
    // multiple pairs
    let mut front = String::new();
    for japanese_word in top.japanese.iter() {
        match *japanese_word {
            Japanese {
                word: Some(ref w),
                reading: Some(ref r),
            } => {
                front.push_str(&format!("{}[{}]  ", w, r));
            }
            // When there is either just the word or the reading, add it
            // without furigana markup
            Japanese {
                word: Some(ref s),
                reading: None,
            }
            | Japanese {
                word: None,
                reading: Some(ref s),
            } => {
                // Note, we still need to add two spaces after the entry
                // so format is still necessary
                front.push_str(&format!("{}  ", s));
            }
            // This shouldn't be possible
            Japanese {
                word: None,
                reading: None,
            } => {
                unreachable!("Japanese word had neither word nor reading field?!");
            }
        }
    }

    // Now construct the definition (back) field
    // I want the final result in Anki to look something like
    // 1. def; def; def;
    // 2. def; def;
    let mut back = String::new();
    for (num, sense) in top.senses.iter().enumerate() {
        back.push_str(&format!("{}. ", num + 1));
        let defs = sense.english_definitions.join("; ");
        back.push_str(&format!("{}<br>", defs));
    }
    Some(Card::new(&front, &back))
}

fn handle_error(search_term: &str, e: reqwest::Error) {
    if e.is_serialization() {
        eprintln!("{}: Failed converting API as JSON {}", search_term, e.get_ref().unwrap());
    } else {
        eprintln!("{}: Failed accessing JishoAPI", search_term);
    }
}
