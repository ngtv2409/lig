mod matcher;
use matcher::{PatternMap, match_files};

mod out;
use out::{print_matches_line, print_count};

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
#[command(about = "Lig is a multi-pattern matching tool")]
pub struct Cli {
    /// The file names to search (`-` : stdin)
    #[arg(default_values_t=vec![String::from("-")])]
    filenames : Vec<String>,

    /// The named regex patterns Name=Regex.
    /// Note: Use --pattern multiple times for multiple patterns
    #[arg(short='e', long="pattern", action=ArgAction::Append, required=true)]
    patterns : Vec<String>,

    /// Invert the sense of matching (non matching)
    #[arg(short='v', long="invert-match", help_heading="Matching control")]
    invert_match: bool,
    /// Ignore the case distinctions of patterns and input data.
    #[arg(short='i', long="ignore-case", help_heading="Matching control")]
    ignore_case: bool,

    /// Enable colorized output
    #[arg(long="color", default_value="auto", help_heading="Output control")]
    color: ColorMode,
    /// Do not print anything beside the headers PATNAME (count)
    #[arg(short='c', long="count", help_heading="Output control")]
    count: bool,

    // Out prefixes
    /// The prefix of each line of output
    #[arg(long="prefix", default_value_t=String::new(), help_heading="Output prefix control")]
    prefix: String,
    /// Follow by file name
    #[arg(short='H', long="with-filename", help_heading="Output prefix control")]
    with_filename: bool,
    /// Follow by line number
    #[arg(short='n', long="line-number", help_heading="Output prefix control")]
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

    if CLI.count {
        print_count(&matches);
    }
    else {
        print_matches_line(&matches);
    }

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
