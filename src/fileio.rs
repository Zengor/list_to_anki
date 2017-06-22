use std::io::{BufReader, BufRead, BufWriter, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;

/// An enum representing a single term read from the file
///
/// Currently irrelevant but will eventually will allow lines to be skipped
/// among other things.
pub enum Term {
    /// Term is to be searched in the dictionary
    Search(String),
    /// Term is not to be searched and added as-is to card file
    Pass(String, String),
}

fn determine_term_type(s: &str) -> Option<Term> {
    if s.is_empty() || s.starts_with('#') {
        return None;
    }
    if s.contains('|') {
        let mut split_s = s.split('|');
        let f = split_s.next().unwrap();
        let b = split_s.next().unwrap();
        Some(Term::Pass(f.into(), b.into()))
    } else {
        Some(Term::Search(s.into()))
    }
}

pub fn read(f_name: &str) -> Vec<Term> {
    let f = File::open(f_name).expect("Failed opening file");
    let lines = BufReader::new(&f).lines();
    let mut out: Vec<Term> = Vec::new();
    for line in lines {
        match determine_term_type(&line.unwrap()) {
            Some(t) => out.push(t),
            None => (),
        }
    }
    out
}

pub fn write(f_name: &str, f_contents: &str, append: bool) {
    let mut options = OpenOptions::new();
    if !Path::new(f_name).exists() {
        options.create(true);
    }
    let f = options.write(true)
        .append(append)
        .truncate(!append)
        .open(f_name).expect("Failed creating/opening file");

    
    // let f = OpenOptions::new()
    //     .create(true)
    //     .append(append)
    //     .open(f_name)
    //     .expect("Failed creating/opening file");
    // let f = File::create(f_name).expect("Failed creating file");
    BufWriter::new(f).write_all(f_contents.as_bytes()).expect("Failed Writing");
}
