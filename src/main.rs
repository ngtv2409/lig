mod matcher;
use matcher::{PatternMap, match_files};

mod out;
use out::{print_matches_line};

mod utils;

use regex::{RegexBuilder};
use colored::control;
use clap::{Parser, ValueEnum, ArgAction};
use anyhow::Result;

use once_cell::sync::Lazy;


#[derive(ValueEnum, Clone, Debug)]
enum ColorMode {
    Never,
    Auto,
    Always,
}

#[derive(Parser, Debug)]
#[command(name = "lig")]
#[command(version = "0.1.0")]
#[command(about = "", long_about = None)]
pub struct Cli {
    #[arg(default_values_t=vec![String::from("-")])]
    filenames : Vec<String>,

    #[arg(long="pattern", action=ArgAction::Append, required=true)]
    patterns : Vec<String>,

    #[arg(short='v', long="invert-match")]
    invert_match: bool,
    #[arg(short='i', long="ignore-case")]
    ignore_case: bool,

    #[arg(long="color", default_value="never")]
    color: ColorMode,

    // Out prefixes
    #[arg(long="prefix", default_value_t=String::new())]
    prefix: String,
    #[arg(short='H', long="with-filename")]
    with_filename: bool,
    #[arg(short='n', long="line-number")]
    line_number: bool,
}

pub static CLI: Lazy<Cli> = Lazy::new(|| Cli::parse());

fn main() -> Result<()> {
    // colored cf 
    match CLI.color {
        ColorMode::Never => control::set_override(false),
        ColorMode::Always => control::set_override(true),
        ColorMode::Auto => {}
    }

    let pmap = parse_patterns(&CLI.patterns)?;
    let matches =
        match_files(
            &CLI.filenames,
            &pmap,
        )?;

    print_matches_line(&matches);

    Ok(())
}


fn parse_patterns(patsr : &Vec<String>) -> Result<PatternMap> {
    let mut map = PatternMap::new();
    for patr in patsr {
        if let Some((key, value)) = patr.split_once('=') {
            let re = RegexBuilder::new(value).case_insensitive(CLI.ignore_case).build()?;
            map.insert(key.to_string(), re);
        } else {
            return Err(anyhow::anyhow!(format!("Invalid pattern '{}', expected KEY=REGEX", patr)));
        }
    }
    Ok(map)
}
