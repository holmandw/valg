#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;
use rand::prelude::*;

const USAGE: &'static str = "
velg chooses one of the inputs

Usage:
  velg <values>... [--all]
  velg (-h | --help)
  velg --version

Options:
  -h --help     Show this screen.
  --all         Show all input values (not just the choice).
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_all: bool,
    arg_values: Vec<String>,
}

fn show_all(vals: &[String], n: usize) {
    for (i, val) in vals.iter().enumerate() {
        let marker = if i == n { "* " } else { "  " };
        println!("{}{}", marker, val);
    }
}

fn show_one(vals: &[String], n: usize) {
    println!("{}", vals[n]);
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    let mut rng = thread_rng();
    let values = args.arg_values;
    let n = rng.gen_range(0, values.len());
    match args.flag_all {
        true => show_all(&values, n),
        false => show_one(&values, n),
    }
}
