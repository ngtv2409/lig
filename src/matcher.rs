/*

    Search in a file using regex and return Match struct containing metadata

*/
use crate::utils::OrdHashMap;

use regex::Regex;
use anyhow::Result;

use std::fs::File;
use std::io::{BufRead, BufReader};


pub type PatternMap = OrdHashMap<String, Regex>;
pub type MatchesMap = OrdHashMap<String, Vec<Line>>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Match {
    pub moffbeg  : usize,  // the line start byte offset of match
    pub moffend  : usize,  // the line end byte offset of match (exclusive)
}

#[derive(Debug)]
pub struct Line {
    pub filename : String,
    pub line     : String,
    pub lineno   : usize,

    pub matches  : Vec<Match>
}

/*
    Match files and return the map
*/
pub fn match_files(fileidents : &Vec<String>,
                      patterns : &PatternMap)
                -> Result<MatchesMap> {
    let mut map = MatchesMap::new();
    // populate map with all patterns
    for patn in &patterns.ord {
        map.insert(patn.to_string(), Vec::new());
    }
        
    for fileident in fileidents {
        let file = File::open(fileident)?;
        let reader = BufReader::new(file);

        for (lineno, line) in reader.lines().enumerate() {
            let line = line?;
            for (patn, re) in &patterns.map {
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
                    });
                }
                if linest.matches.len() > 0 {
                    map.map.get_mut(patn.as_str()).unwrap().push(linest);
                }
            }
        }
    }
    Ok(map)
}
