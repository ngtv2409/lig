mod matcher;
use matcher::{PatternMap, match_files};

mod out;
use out::{
    print_count, print_files_with_matches, print_files_without_match, print_matches,
    print_matches_line,
};

use anyhow::Result;
use clap::{ArgAction, Parser, ValueEnum};
use colored::control;
use regex::RegexBuilder;

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
    filenames: Vec<String>,

    /// The named regex patterns Name=Regex.
    /// Note: Use --pattern multiple times for multiple patterns
    #[arg(short='e', long="pattern", action=ArgAction::Append, required=true)]
    patterns: Vec<String>,

    /// Invert the sense of matching (non matching)
    #[arg(short = 'v', long = "invert-match", help_heading = "Matching control")]
    invert_match: bool,
    /// Ignore the case distinctions of patterns and input data.
    #[arg(short = 'i', long = "ignore-case", help_heading = "Matching control")]
    ignore_case: bool,

    /// Enable colorized output
    #[arg(
        long = "color",
        default_value = "auto",
        help_heading = "Output control"
    )]
    color: ColorMode,
    /// Suppress normal output. Print the heading (overrides hide-heading)
    #[arg(short = 'c', long = "count", help_heading = "Output control")]
    count: bool,
    /// Suppress normal output. Print file names with matches
    #[arg(
        short = 'l',
        long = "files-with-matches",
        help_heading = "Output control"
    )]
    files_with_matches: bool,
    /// Suppress normal output. Print file names without matches
    #[arg(
        short = 'L',
        long = "files-without-match",
        help_heading = "Output control"
    )]
    files_without_match: bool,
    /// Print only the matching parts of the line
    #[arg(short = 'o', long = "only-matching",
        help_heading = "Output control"
    )]
    only_matching: bool,

    // Out prefixes
    /// The prefix of each line of output
    #[arg(long="prefix", default_value_t=String::new(), help_heading="Output prefix control")]
    prefix: String,
    /// include file name
    #[arg(
        short = 'H',
        long = "with-filename",
        help_heading = "Output prefix control"
    )]
    with_filename: bool,
    /// include line number
    #[arg(
        short = 'n',
        long = "line-number",
        help_heading = "Output prefix control"
    )]
    line_number: bool,
    /// hide heading PATN (COUNT)
    #[arg(short = 'C', long = "hide-heading",
        help_heading = "Output prefix control"
    )]
    hide_heading: bool,
    /// include pattern name
    #[arg(short = 'P', long = "show-pattern",
        help_heading = "Output prefix control"
    )]
    show_pattern: bool,
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
    let matches = match_files(&CLI.filenames, &pmap)?;

    if CLI.count {
        print_count(&matches);
    } else if CLI.files_with_matches {
        print_files_with_matches(&matches);
    } else if CLI.files_without_match {
        print_files_without_match(&matches);
    } else if CLI.only_matching {
        print_matches(&matches);
    } else {
        print_matches_line(&matches);
    }

    Ok(())
}

fn parse_patterns(patsr: &Vec<String>) -> Result<PatternMap> {
    let mut map = PatternMap::new();
    for patr in patsr {
        if let Some((key, value)) = patr.split_once('=') {
            let re = RegexBuilder::new(value)
                .case_insensitive(CLI.ignore_case)
                .build()?;
            map.insert(key.to_string(), re);
        } else {
            return Err(anyhow::anyhow!(format!(
                "Invalid pattern '{}', expected KEY=REGEX",
                patr
            )));
        }
    }
    Ok(map)
}
