use clap::Parser;
use glob::glob;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    patterns: Vec<String>
}

fn main() {
    let args = Args::parse();

    for pattern in &args.patterns {
        let paths = glob(pattern)
            .expect("invalid pattern")
            .flatten();

        for path in paths {
            println!("{path:?}")
        }
    }
}
