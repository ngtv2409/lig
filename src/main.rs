mod matcher;
use matcher::{PatternMap, match_files};

mod out;
use out::{OutOptions, print_matches_line};

mod utils;

use regex::Regex;
use colored::control;
use clap::{Parser, ValueEnum};
use anyhow::Result;


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
struct Cli {
    filenames : Vec<String>,

    #[arg(long="pattern", default_values_t=vec![String::from("*=.*")])]
    patterns : Vec<String>,

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    dbg!(&cli);

    // colored cf 
    match cli.color {
        ColorMode::Never => control::set_override(false),
        ColorMode::Always => control::set_override(true),
        ColorMode::Auto => {}
    }

    let outopts = OutOptions {
        prefix : cli.prefix,
        show_filename : cli.with_filename,
        show_linenumber : cli.line_number,
        ..Default::default()
    };

    let pmap = parse_patterns(&cli.patterns)?;
    dbg!(&pmap);
    let matches =
        match_files(
            &cli.filenames,
            &pmap,
        )?;
    dbg!(&matches);

    print_matches_line(&matches, &outopts);

    Ok(())
}


fn parse_patterns(patsr : &Vec<String>) -> Result<PatternMap> {
    let mut map = PatternMap::new();
    for patr in patsr {
        if let Some((key, value)) = patr.split_once('=') {
            let re = Regex::new(value)?;
            map.insert(key.to_string(), re);
        } else {
            return Err(anyhow::anyhow!(format!("Invalid pattern '{}', expected KEY=REGEX", patr)));
        }
    }
    Ok(map)
}
