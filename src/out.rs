use crate::matcher::{Match, MatchesMap};

use colored::{Colorize, Color};

pub struct OutOptions {
    // prefixes
    pub prefix : String,

    pub show_filename : bool,
    pub show_linenumber : bool,
}
impl OutOptions {
    fn format_prefix(&self, filename : &str, lineno : usize) -> String {
        let mut s = self.prefix.clone();
        if self.show_filename {
            s.push_str(&format!("{}:", filename).magenta().to_string());
        }
        if self.show_linenumber {
            s.push_str(&format!("{}:", lineno + 1).cyan().to_string());
        }
        s
    }
}
impl Default for OutOptions {
    fn default() -> Self {
        Self {
            prefix : "".to_string(),

            show_filename: false,
            show_linenumber: false,
        }
    }
}

/*
    Normal default output mode

    Prints the entire line with matches
*/
pub fn print_matches_line(pats : &MatchesMap, opts : &OutOptions) {
    for pat in &pats.ord {
        // lines should always exists because OrdMap always insert in pair 
        // this is just guardrail
        if let Some(lines) = pats.map.get(pat.as_str()) {
            println!("{} ({})", format!("{}", pat).yellow().bold(), lines.len());
            for line in lines {
                println!("{}{}", opts.format_prefix(&line.filename, line.lineno),
                        highlight_matches(&line.line, &line.matches, Color::Red));
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
