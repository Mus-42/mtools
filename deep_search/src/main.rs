use clap::Parser;
use glob::glob;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short)]
    display_zero_matches: bool,

    seach_pattern: String,
    files: Vec<String>
}

fn main() {
    let args = Args::parse();

    let search = unescape(&args.seach_pattern).unwrap();

    println!("searching for: `{}` unescaped as: `{search:02X?}`", &args.seach_pattern);

    for file_patt in &args.files {
        let paths = glob(file_patt)
            .expect("invalid glob pattern")
            .flatten();

        for path in paths {
            let Ok(data) = std::fs::read(path.as_path()) else {
                println!("can't read `{path:?}`");
                continue;
            };

            let mut num_matches = 0u32;

            for window in data.windows(search.len()) {
                if window == search {
                    num_matches += 1;
                }
            }

            if args.display_zero_matches || num_matches > 0 {
                println!("`{path:?}` - {num_matches} matches");
            }
        }
    }
}

fn unescape(s: &str) -> Option<Vec<u8>> {
    let mut chars = s.chars();
    let mut s = Vec::with_capacity(s.len());

    while let Some(ch) = chars.next() {
        // ascii-only input. for now.
        if !ch.is_ascii() {
            return None;
        }

        if ch != '\\' {
            s.push(ch as u8);
            continue;
        }

        let ch = chars.next()?;
        let ch = match ch {
            'b' => 0x08,
            'f' => 0x0c,
            'n' => b'\n',
            'r' => b'\r',
            't' => b'\t',

            '\'' | '\"' | '\\' | '/' => ch as u8,

            'x' => {
                let a = chars.next()?.to_digit(16)?;
                let b = chars.next()?.to_digit(16)?;

                let byte = (a*16 + b) as u8;

                s.push(byte);
                continue;
            }

            _ => return None
        };

        s.push(ch);
    }

    Some(s)
}