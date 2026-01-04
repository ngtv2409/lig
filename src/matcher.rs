/*

    Search in a file using regex and return Match struct containing metadata

*/
use crate::CLI;

use anyhow::Result;
use indexmap::IndexMap;
use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

pub type PatternMap = IndexMap<String, Regex>;
pub type MatchesMap = IndexMap<String, Vec<Line>>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Match {
    pub moffbeg: usize, // the line start byte offset of match
    pub moffend: usize, // the line end byte offset of match (exclusive)
}

#[derive(Debug)]
pub struct Line {
    pub filename: String,
    pub line: String,
    pub lineno: usize,

    pub matches: Vec<Match>,
}

/*
    Match files and return the map
*/
pub fn match_files(fileidents: &Vec<String>, patterns: &PatternMap) -> Result<MatchesMap> {
    let mut map = MatchesMap::new();

    for fileident in fileidents {
        if fileident.as_str() != "-" {
            let file = File::open(fileident)?;
            let reader = BufReader::new(file);
            map = matcher(&fileident, reader, &patterns, map)?;
        } else {
            map = matcher("(standard input)", io::stdin(), &patterns, map)?;
        }
    }
    Ok(map)
}

fn matcher<R: Read>(
    fileident: &str,
    read: R,
    patterns: &PatternMap,
    mut map: MatchesMap,
) -> Result<MatchesMap> {
    let reader = BufReader::new(read);
    for (lineno, line) in reader.lines().enumerate() {
        let line = line?;
        for (patn, re) in patterns {
            let mut linest: Line = Line {
                filename: fileident.to_string(),
                line: line.clone(),
                lineno: lineno,
                matches: Vec::<Match>::new(),
            };
            for m in re.find_iter(&line) {
                linest.matches.push(Match {
                    moffbeg: m.start(),
                    moffend: m.end(),
                });
            }
            // when invert is true, xor inverses the condition,
            // so len == 0 (no match)
            if (linest.matches.len() > 0) ^ CLI.invert_match {
                map.entry(patn.to_string())
                    .or_insert(Vec::new())
                    .push(linest);
            }
        }
    }
    Ok(map)
}
