Lig is a file pattern matching tool with named pattern

Version: 0.1.0 (Prototype)
Many things are missing but it is functional.

```sh
$ cargo -q run src/main.rs -HNn --prefix="  " --pattern FuncDecl=fn --pattern VarDecl=let
FuncDecl (2 matches)
  src/main.rs:40:1:fn main() -> io::Result<()> {
  src/main.rs:73:1:fn parse_patterns(patsr : &Vec<String>) -> Result<PatternMap, String> {
VarDecl (9 matches)
  src/main.rs:41:5:    let cli = Cli::parse();
  src/main.rs:43:5:    let pmap = parse_patterns(&cli.patterns).expect("Failed to parse pattern");
  src/main.rs:45:5:    let mut matches = HashMap::<String, Vec<Line>>::new();
  src/main.rs:48:9:        let file = File::open(&filename)?;
  src/main.rs:49:9:        let reader = BufReader::new(file);
  src/main.rs:59:5:    let outopts = OutOptions {
  src/main.rs:74:5:    let mut map = PatternMap::new();
  src/main.rs:76:12:        if let Some((key, value)) = patr.split_once('=') {
  src/main.rs:77:13:            let re = Regex::new(value).expect("Failed to parse regex");
```
