use clap::Clap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clap, Debug)]
#[clap(
    name = "my rpn program",
    version = "1.0.0",
    author = "uegusi",
    about = "calc"
)]
struct Opts {
    ///formula written in rpn
    #[clap(short, long)]
    verbose: bool,
    ///sets ths level of verbosity
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    } else {
        println!("No file specified.");
    }
}
