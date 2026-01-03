/*

    Search in a file using regex and return Match struct containing metadata

*/
use crate::utils::OrdHashMap;

use regex::Regex;
use anyhow::Result;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};


#[derive(Debug)]
pub struct MatchOptions {
    pub invert : bool
}

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
                      patterns : &PatternMap, opts : &MatchOptions)
                -> Result<MatchesMap> {
    let mut map = MatchesMap::new();
    // populate map with all patterns
    for patn in &patterns.ord {
        map.insert(patn.to_string(), Vec::new());
    }
        
    for fileident in fileidents {
        if fileident.as_str() != "-" {
            let file = File::open(fileident)?;
            let reader = BufReader::new(file);
            map = matcher(&fileident, reader, &patterns, map, opts)?;
        } else {
            map = matcher("(standard input)", io::stdin(), &patterns, map, opts)?;
        }

    }
    Ok(map)
}

/* Helper function for matching logic
    Premise: All entries exist: patterns.keys is subset of matches.keys
*/
fn matcher<R: Read>(fileident : &str, read : R,
               patterns : &PatternMap,
               mut map : MatchesMap, opts : &MatchOptions) -> Result<MatchesMap> {
    let reader = BufReader::new(read);
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
            // when invert is true, xor inverses the condition, 
            // so len == 0 (no match)
            if (linest.matches.len() > 0) ^ opts.invert {
                map.map.get_mut(patn.as_str()).unwrap().push(linest);
            }
        }
    }
    Ok(map)
}
