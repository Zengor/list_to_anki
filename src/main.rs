mod cards;
mod fileio;
mod jisho;

use cards::Card;
use clap::{App, Arg};
use fileio::Term;
use rayon::prelude::*;

fn main() {
    let arg_matches = App::new("list_to_anki")
        .author("Zengor")
        .version("0.2")
        .arg(
            Arg::with_name("file")
                .value_name("FILE_NAME")
                .help("Path to list file")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE_NAME")
                .help("Specifices the output deck file name. Default is [input]_generated_deck"),
        )
        .arg(
            Arg::with_name("append")
                .short("a")
                .long("append")
                .help("Appends to output file instead of replacing it"),
        )
        .arg(
            Arg::with_name("single-threaded")
                .long("singlet")
                .help("Runs the search in a single thread. Should be really slow"),
        )
        .get_matches();

    let f = arg_matches
        .value_of("file")
        .expect("File path not recognized");
    let out: String = match arg_matches.value_of("output") {
        Some(o) => o.to_owned(),
        None => format!("{}_generated_deck", f),
    };

    // Read file
    let lines: Vec<Term> = fileio::read(f);
    // Generate cards
    let cards: Vec<Card> = {
        if arg_matches.is_present("single-threaded") {
            lines
                .iter()
                .filter_map(|t| cards::generate_card(t))
                .collect()
        } else {
            lines
                .par_iter()
                .filter_map(|t| cards::generate_card(t))
                .collect()
        }
    };
    // Output file
    let mut out_file = String::new();
    for card in cards.iter() {
        out_file.push_str(&format!("{}\n", card));
    }
    fileio::write(&out, &out_file, arg_matches.is_present("append"));
}
