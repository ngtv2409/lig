mod search;
use search::{PatternMap, Line, match_file};

mod out;
use out::{OutOptions, print_matches};

use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader};
use std::collections::HashMap;

use clap::Parser;

#[derive(Parser)]
#[command(name = "lig")]
#[command(version = "0.1.0")]
#[command(about = "", long_about = None)]
struct Cli {
    filenames : Vec<String>,

    // Out prefixes
    #[arg(short='H', long="with-filename")]
    with_filename: bool,
    #[arg(short='n', long="line-number")]
    line_number: bool,
    #[arg(short='c', long="col-number")]
    col_number: bool,

}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut pmap = PatternMap::new();
    pmap.insert("FunctionDecl".to_string(), Regex::new(r"\bfn\b").unwrap());
    pmap.insert("VarDecl".to_string(), Regex::new(r"\blet\b").unwrap());

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

    let outopts = OutOptions {
        show_filename : cli.with_filename,
        show_linenumber : cli.line_number,
        show_colnumber : cli.col_number,
        ..Default::default()
    };
    print_matches(&matches, &outopts);

    Ok(())
}
