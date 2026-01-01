/*

    Search in a file using regex and return Match struct containing metadata

*/
use regex::Regex;
use std::io::BufRead;
use std::collections::HashMap;

pub type PatternMap = HashMap<String, Regex>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Match {
    pub moffbeg  : usize,  // the line start byte offset of match
    pub moffend  : usize,  // the line end byte offset of match (exclusive)

    pub patname  : String  // name of the pattern matched   
}

#[derive(Debug)]
pub struct Line {
    pub line    : String,
    pub lineno  : usize,

    pub matches : HashMap<String, Vec<Match>>
}

/* An association of filename and matches */

pub struct FileMatch {
    pub filename : String, pub lines : Vec<Line>
}


/*
    Promise: Return vector is sorted by lineno
*/
pub fn match_file<F : BufRead>(reader : F, patterns : &PatternMap) -> Vec<Line> {
    let mut linests : Vec<Line> = Vec::new();

    for (lineno, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        let mut linest : Line = Line {
            line    : line.clone(),
            lineno  : lineno,
            matches : HashMap::<String, Vec<Match>>::new()
        };
        for (patn, re) in patterns {
            for m in re.find_iter(&line) {
                linest.matches.entry(patn.clone())
                    .or_insert_with(Vec::new)
                    .push(Match {
                        moffbeg : m.start(),
                        moffend : m.end(),
                        patname : patn.clone()
                    });
            }
        }
        if linest.matches.len() > 0 {
            linests.push(linest);
        }
    }

    linests
}
