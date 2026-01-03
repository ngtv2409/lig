use crate::matcher::{Match, MatchesMap};
use crate::CLI;

use colored::{Colorize, Color};

fn format_prefix(filename : &str, lineno : usize, ifn : bool, iln : bool) -> String {
    let mut s = CLI.prefix.clone();
    if ifn {
        s.push_str(&format!("{}:", filename).magenta().to_string());
    }
    if iln {
        s.push_str(&format!("{}:", lineno + 1).cyan().to_string());
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
                println!("{}{}", format_prefix(&line.filename, line.lineno, CLI.with_filename, CLI.line_number),
                        highlight_matches(&line.line, &line.matches, Color::Red));
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
                        println!("{}{}", format_prefix(p, 0, CLI.with_filename, false), count);
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
                println!("{}{}", format_prefix(p, 0, CLI.with_filename, false), count);
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
