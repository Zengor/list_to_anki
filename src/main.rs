
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
        .version("0.2")
        .arg(Arg::with_name("file")
             .value_name("FILE_NAME")
             .help("Path to list file")
             .required(true))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("FILE_NAME")
             .help("Specifices the output deck file name. Default is [input]_generated_deck"))
        .get_matches();
    
    let f = arg_matches.value_of("file").expect("File path not recognized");
    let out: String = match arg_matches.value_of("output") {
        Some(o) => o.to_owned(),
        None => format!("{}_generated_deck", f)        
    };
             
    // Read file
    let lines: Vec<Term> = fileio::read(f);
    /// / Generate cards
    let cards: Vec<Card> = lines.par_iter().filter_map(|t| cards::generate_card(t)).collect();
    // Output file
    let mut out_file = String::new();
    for card in cards.iter() {
        out_file.push_str(&format!("{}\n", card));
    }
    fileio::write(&out, &out_file);
}
