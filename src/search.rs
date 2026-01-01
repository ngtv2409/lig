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
    pub filename : String,
    pub line     : String,
    pub lineno   : usize,

    pub matches  : Vec<Match>
}

/*
    Match a file and update the map
*/
pub fn match_file<'a, F : BufRead>(reader : F, fileident : &str,
                               map : &'a mut HashMap<String, Vec<Line>>,
                               patterns : &PatternMap)
                -> &'a mut HashMap<String, Vec<Line>> {
    for (lineno, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        for (patn, re) in patterns {
            let mut linest : Line = Line {
                filename: fileident.to_string(),
                line    : line.clone(),
                lineno  : lineno,
                matches : Vec::<Match>::new()
            };
            for m in re.find_iter(&line) {
                linest.matches.push(Match {
                        moffbeg : m.start(),
                        moffend : m.end(),
                        patname : patn.clone()
                });
            }
            if linest.matches.len() > 0 {
                map.entry(patn.clone())
                    .or_insert_with(Vec::new)
                    .push(linest);
            }
        }
    }
    map
}
