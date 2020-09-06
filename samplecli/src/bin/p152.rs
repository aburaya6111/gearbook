use clap::Clap;

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
    match opts.formula_file {
        Some(file) => println!("File specified:{}", file),
        None => println!("No file specified."),
    }
    let verbose = opts.verbose;
    println!("Is verbosity specified:{}", verbose);
}
