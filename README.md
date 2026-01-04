Lig is a file pattern matching tool with named pattern

Version: 1.0.0

```sh
cargo install lig-rs
```

```
> cargo run -qr -- src/* -e "Function=^\bfn\s*\w+\(.*?\)" -e "Var=let ([^=]+)" -HPno --prefix='  '
Function (5)
  Function:src/main.rs:107:fn main()
  Function:src/main.rs:133:fn parse_patterns(patsr: &Vec<String>)
  Function:src/out.rs:25:fn format_prefix(filename: &str, lineno: usize, patn: &str, o: PrefixFormatOptions)
  Function:src/out.rs:39:fn print_heading(pat: &str, count: usize)
  Function:src/out.rs:202:fn highlight_matches(text: &str, matches: &[Match], color: Color)
Var (22)
  Var:src/main.rs:115:let pmap
  Var:src/main.rs:116:let matches
  Var:src/main.rs:134:let mut map
  Var:src/main.rs:136:let Some((key, value))
  Var:src/main.rs:137:let re
  Var:src/matcher.rs:38:let mut map
  Var:src/matcher.rs:45:let file
  Var:src/matcher.rs:46:let reader
  Var:src/matcher.rs:61:let reader
  Var:src/matcher.rs:63:let line
  Var:src/matcher.rs:65:let mut linest: Line
  Var:src/out.rs:26:let sep
  Var:src/out.rs:27:let mut s
  Var:src/out.rs:97:let mut prev: Option<&str>
  Var:src/out.rs:98:let mut count
  Var:src/out.rs:101:let fname
  Var:src/out.rs:129:let Some(p)
  Var:src/out.rs:149:let mut prev_file: Option<&str>
  Var:src/out.rs:176:let mut seen
  Var:src/out.rs:203:let mut result
  Var:src/out.rs:204:let mut last_index
  Var:src/out.rs:212:let matched
```

```
Lig is a multi-pattern matching tool

Usage: lig [OPTIONS] --pattern <PATTERNS> [FILENAMES]...

Arguments:
  [FILENAMES]...  The file names to search (`-` : stdin) [default: -]

Options:
  -e, --pattern <PATTERNS>  The named regex patterns Name=Regex. Note: Use --pattern multiple times for multiple patterns
  -h, --help                Print help
  -V, --version             Print version

Matching control:
  -v, --invert-match  Invert the sense of matching (non matching)
  -i, --ignore-case   Ignore the case distinctions of patterns and input data

Output control:
      --color <COLOR>        Enable colorized output [default: auto] [possible values: never, auto, always]
  -c, --count                Suppress normal output. Print the heading (overrides hide-heading)
  -l, --files-with-matches   Suppress normal output. Print file names with matches
  -L, --files-without-match  Suppress normal output. Print file names without matches
  -o, --only-matching        Print only the matching parts of the line

Output prefix control:
      --prefix <PREFIX>  The prefix of each line of output [default: ]
  -H, --with-filename    include file name
  -n, --line-number      include line number
  -C, --hide-heading     hide heading PATN (COUNT)
  -P, --show-pattern     include pattern name
```

TODO: add complete README
