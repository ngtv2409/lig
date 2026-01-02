use crate::matcher::{Line};
use std::collections::HashMap;

pub struct OutOptions {
    // out opts
    pub count: bool,

    // prefixes
    pub prefix : String,

    pub show_filename : bool,
    pub show_linenumber : bool,
    pub show_colnumber : bool
}
impl Default for OutOptions {
    fn default() -> Self {
        Self {
            count: false,

            prefix : "".to_string(),

            show_filename: false,
            show_linenumber: false,
            show_colnumber: false
        }
    }
}

pub fn print_matches(pats : &HashMap<String, Vec<Line>>, opts : &OutOptions) {
    for (pat, lines) in pats.iter() {
        println!("{} ({} matches)", pat, lines.len());
        if !opts.count {
            for line in lines {
                for m in &line.matches {
                    println!("{}{}{}",
                        opts.prefix,
                        format_location(&line.filename, line.lineno, 
                            byte_to_char_offset(&line.line, m.moffbeg), &opts),
                        line.line
                    );
                }
            }
        }
    }
}

fn byte_to_char_offset(s: &str, byte_offset: usize) -> usize {
    s[..byte_offset].chars().count()
}

fn format_location(filename : &str, lineno : usize, colno : usize,
                   opts : &OutOptions) -> String {
    let mut s : String = String::new();

    if opts.show_filename {
        s += &format!("{}:", filename);
    }
    if opts.show_linenumber {
        s += &format!("{}:", lineno + 1);
    }
    if opts.show_colnumber {
        s += &format!("{}:", colno + 1);
    }

    s
}
