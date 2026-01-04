use crate::matcher::{Match, MatchesMap};
use crate::CLI;

use colored::{Colorize, Color};

use std::collections::HashSet;

struct PrefixFormatOptions {
    pub ifn : bool,
    pub iln : bool,
    pub ipatn : bool,
    pub isep : bool,
}
impl Default for PrefixFormatOptions {
    fn default() -> Self {
        // default to CLI, overwrite when need
        PrefixFormatOptions {
            ifn: CLI.with_filename,
            iln: CLI.line_number,
            ipatn: CLI.show_pattern,
            isep: true
        }
    }
}
fn format_prefix(filename : &str, lineno : usize, patn : &str,
                o: PrefixFormatOptions) -> String {
    let sep = if o.isep {
        ":"
    } else {
        ""
    };
    let mut s = CLI.prefix.clone();
    if o.ipatn {
        s.push_str(&format!("{}{}", patn, sep).yellow().to_string());
    }
    if o.ifn {
        s.push_str(&format!("{}{}", filename, sep).magenta().to_string());
    }
    if o.iln {
        s.push_str(&format!("{}{}", lineno + 1, sep).cyan().to_string());
    }
    s
}
fn print_heading(pat: &str, count: usize) {
    if !CLI.hide_heading {
        if CLI.invert_match {
            print!("{}", "!".red())
        }
        println!("{} ({})",
            format!("{}", pat).yellow().bold(), count);
    }
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
            print_heading(pat, lines.len());
            for line in lines {
                println!("{}{}", format_prefix(&line.filename, line.lineno, pat,
                                PrefixFormatOptions{..Default::default()}),
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
            print_heading(pat, lines.len());
            for line in lines {
                for m in &line.matches {
                    println!("{}{}", format_prefix(&line.filename, line.lineno, pat,
                                PrefixFormatOptions {..Default::default()}),
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
                        println!("{}{}", format_prefix(p, 0, "", PrefixFormatOptions{iln: false, ..Default::default()}), count);
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
                println!("{}{}", format_prefix(p, 0, "", PrefixFormatOptions{iln: false, ..Default::default()}), count);
            }
        }
    }
}
pub fn print_files_with_matches(matches : &MatchesMap) {
    for pat in &matches.ord {
        if let Some(lines) = matches.map.get(pat.as_str()) {
            print_heading(pat, lines.len());
            let mut prev_file: Option<&str> = None;

            for line in lines {
                if prev_file != Some(line.filename.as_str()) {
                    println!("{}", format_prefix(&line.filename, 0, &(pat.to_owned()+":"),
                            PrefixFormatOptions {ifn:true, iln:false, isep:false,..Default::default()}));
                }
                prev_file = Some(line.filename.as_str());
            }
        }
    }
}
pub fn print_files_without_match(matches : &MatchesMap) {
    for pat in &matches.ord {
        if let Some(lines) = matches.map.get(pat.as_str()) {
            print_heading(pat, lines.len());

            let mut seen = HashSet::<String>::new();
            for line in lines {
                seen.insert(line.filename.clone());
            }

            for file in &CLI.filenames {
                if ! seen.contains(file) {
                    println!("{}", format_prefix(file, 0, &(pat.to_owned()+":"),
                            PrefixFormatOptions {ifn:true, iln:false, isep:false,..Default::default()}));
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
