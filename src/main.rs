
extern crate clap;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rayon;
mod fileio;
mod cards;
mod jisho;

use clap::{Arg, App};
use fileio::Term;
use cards::Card;
use rayon::prelude::*;

fn main() {
    let arg_matches = App::new("list_to_anki")
        .author("Zengor")
        .arg(Arg::with_name("file")
            .value_name("FILE_NAME")
            .help("Path to list file"))
        .get_matches();
    let f = arg_matches.value_of("file").expect("File path not recognized");
    // Read file
    let lines: Vec<Term> = fileio::read(f);
    /// / Generate cards
    let cards: Vec<Card> = lines.par_iter().filter_map(|t| cards::generate_card(t)).collect();
    // Output file
    let mut out_file = String::new();
    for card in cards.iter() {
        out_file.push_str(&format!("{}\n", card));
    }
    let out_f_name = format!("{}_generated_deck", f);
    fileio::write(&out_f_name, &out_file);
}
