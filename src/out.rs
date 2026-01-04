use crate::matcher::{Match, MatchesMap};
use crate::CLI;

use colored::{Colorize, Color};

use std::collections::HashSet;

fn format_prefix(filename : &str, lineno : usize, ifn : bool, iln : bool, isep : bool) -> String {
    let sep = if isep {
        ":"
    } else {
        ""
    };
    let mut s = CLI.prefix.clone();
    if ifn {
        s.push_str(&format!("{}{}", filename, sep).magenta().to_string());
    }
    if iln {
        s.push_str(&format!("{}{}", lineno + 1, sep).cyan().to_string());
    }
    s
}

/*
    Normal default output mode

    Prints the entire line with matches
*/
pub fn print_matches_line(pats : &MatchesMap) {
    for pat in &pats.ord {
        // lines should always exists because OrdMap always insert in pair 
        // this is just guardrail
        if let Some(lines) = pats.map.get(pat.as_str()) {
            if CLI.invert_match {
                print!("{}", "!".red())
            }
            println!("{} ({})", format!("{}", pat).yellow().bold(), lines.len());
            for line in lines {
                println!("{}{}", format_prefix(&line.filename, line.lineno, CLI.with_filename, CLI.line_number, true),
                        highlight_matches(&line.line, &line.matches, Color::Red));
            }
        }
    }
}
pub fn print_matches(pats : &MatchesMap) {
    for pat in &pats.ord {
        // lines should always exists because OrdMap always insert in pair 
        // this is just guardrail
        if let Some(lines) = pats.map.get(pat.as_str()) {
            if CLI.invert_match {
                print!("{}", "!".red())
            }
            println!("{} ({})", format!("{}", pat).yellow().bold(), lines.len());
            for line in lines {
                for m in &line.matches {
                    println!("{}{}", format_prefix(&line.filename, line.lineno, CLI.with_filename, CLI.line_number, true),
                            line.line[m.moffbeg..m.moffend].red());
                }
            }
        }
    }
}
pub fn print_count(pats : &MatchesMap) {
    for pat in &pats.ord {
        // lines should always exists because OrdMap always insert in pair 
        // this is just guardrail
        if let Some(lines) = pats.map.get(pat.as_str()) {
            if CLI.invert_match {
                print!("{}", "!".red())
            }
            println!("{} ({})", format!("{}", pat).yellow().bold(), lines.len());
            let mut prev: Option<&str> = None;
            let mut count = 0;

            for line in lines {
                let fname = line.filename.as_str();

                match prev {
                    Some(p) if p == fname => count += 1,
                    Some(p) => {
                        println!("{}{}", format_prefix(p, 0, CLI.with_filename, false, true), count);
                        count = 1;
                        prev = Some(fname);
                    }
                    None => {
                        count = 1;
                        prev = Some(fname);
                    }
                }
            }

            if let Some(p) = prev {
                println!("{}{}", format_prefix(p, 0, CLI.with_filename, false, true), count);
            }
        }
    }
}
pub fn print_files_with_matches(matches : &MatchesMap) {
    for pat in &matches.ord {
        if let Some(lines) = matches.map.get(pat.as_str()) {
            if CLI.invert_match {
                print!("{}", "!".red())
            }
            println!("{} ({})", format!("{}", pat).yellow().bold(), lines.len());
            let mut prev_file: Option<&str> = None;

            for line in lines {
                if prev_file != Some(line.filename.as_str()) {
                    println!("{}", format_prefix(&line.filename, 0, true, false, false));
                }
                prev_file = Some(line.filename.as_str());
            }
        }
    }
}
pub fn print_files_without_match(matches : &MatchesMap) {
    for pat in &matches.ord {
        if let Some(lines) = matches.map.get(pat.as_str()) {
            if CLI.invert_match {
                print!("{}", "!".red())
            }
            println!("{} ({})", format!("{}", pat).yellow().bold(), lines.len());

            let mut seen = HashSet::<String>::new();
            for line in lines {
                seen.insert(line.filename.clone());
            }

            for file in &CLI.filenames {
                if ! seen.contains(file) {
                    println!("{}", format_prefix(file, 0, true, false, false));
                }
            }
        }
    }
}


fn highlight_matches(text: &str, matches: &[Match], color: Color) -> String {
    let mut result = String::new();
    let mut last_index = 0;

    for m in matches {
        if last_index < m.moffbeg {
            result.push_str(&text[last_index..m.moffbeg]);
        }

        if m.moffbeg < m.moffend {
            let matched = &text[m.moffbeg..m.moffend];
            result.push_str(&matched.color(color).to_string());
        }

        last_index = m.moffend;
    }

    if last_index < text.len() {
        result.push_str(&text[last_index..]);
    }

    result
}
