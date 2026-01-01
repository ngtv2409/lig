/*

    Search in a file using regex and return Match struct containing metadata

*/
use regex::Regex;
use std::io::BufRead;
use std::collections::HashMap;

pub type PatternMap = HashMap<String, Regex>;

/* An association of filename and matches */
#[derive(Debug)]
pub struct FileMatch{
    pub filename : String, pub match : Match
};

#[derive(Debug)]
pub struct Match {
    pub line     : String, // lile string
    pub lineno   : usize,  // line number in file
    pub moffbeg  : usize,  // the line start byte offset of match
    pub moffend  : usize,  // the line end byte offset of match (exclusive)

    pub patname  : String  // name of the pattern matched   
}

/*
    Promise: Return vector is sorted by lineno
*/
pub fn match_file<F : BufRead>(reader : F, patterns : &PatternMap) -> Vec<Match> {
    let mut ms : Vec<Match> = Vec::new();

    for (lineno, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        for (patn, re) in patterns {
            for m in re.find_iter(&line) {
                ms.push(Match {
                    line    : line.clone(),
                    lineno  : lineno,
                    moffbeg : m.start(),
                    moffend : m.end(),
                    patname : patn.clone()
                });
            }
        }
    }

    ms
}
