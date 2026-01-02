mod search;
use search::{PatternMap, Line, match_file};

mod out;
use out::{OutOptions, print_matches};

use regex::Regex;
use colored::control;

use std::fs::File;
use std::io::{self, BufReader};
use std::collections::HashMap;

use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone)]
enum ColorMode {
    Never,
    Auto,
    Always,
}

#[derive(Parser)]
#[command(name = "lig")]
#[command(version = "0.1.0")]
#[command(about = "", long_about = None)]
struct Cli {
    filenames : Vec<String>,

    #[arg(long="pattern", default_values_t=vec![String::from("*=.*")])]
    patterns : Vec<String>,

    // Output control
    #[arg(short='c', long="count")]
    count: bool,

    #[arg(long="color", default_value="never")]
    color: ColorMode,

    // Out prefixes
    #[arg(long="prefix", default_value_t=String::new())]
    prefix: String,
    #[arg(short='H', long="with-filename")]
    with_filename: bool,
    #[arg(short='n', long="line-number")]
    line_number: bool,
    #[arg(short='N', long="col-number")]
    col_number: bool,

}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let pmap = parse_patterns(&cli.patterns).expect("Failed to parse pattern");

    let mut matches = HashMap::<String, Vec<Line>>::new();

    for filename in cli.filenames {
        let file = File::open(&filename)?;
        let reader = BufReader::new(file);
        
        match_file(
            reader,
            &filename,
            &mut matches,
            &pmap,
        );
    }

    // colored cf 
    match cli.color {
        ColorMode::Never => control::set_override(false),
        ColorMode::Always => control::set_override(true),
        ColorMode::Auto => {}
    }

    let outopts = OutOptions {
        count : cli.count,
        prefix : cli.prefix,
        show_filename : cli.with_filename,
        show_linenumber : cli.line_number,
        show_colnumber : cli.col_number,
        ..Default::default()
    };
    print_matches(&matches, &outopts);

    Ok(())
}


fn parse_patterns(patsr : &Vec<String>) -> Result<PatternMap, String> {
    let mut map = PatternMap::new();
    for patr in patsr {
        if let Some((key, value)) = patr.split_once('=') {
            let re = Regex::new(value).expect("Failed to parse regex");
            map.insert(key.to_string(), re);
        } else {
            return Err(format!("Invalid pattern '{}', expected KEY=REGEX", patr));
        }
    }
    Ok(map)
}
