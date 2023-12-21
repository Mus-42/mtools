use clap::Parser;
use glob::glob;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short)]
    display_per_file: bool,

    patterns: Vec<String>
}

#[derive(Debug, Clone, Copy)]
enum ProgramminLang {
    CLike,
    Rust,
    Other,
}

fn main() {
    let args = Args::parse();

    let mut loc_total = 0;
    let mut lloc_total = 0;

    for pattern in &args.patterns {
        let paths = glob(pattern)
            .expect("invalid pattern")
            .flatten();

        for path in paths {
            if path.is_file() {
                let src = std::fs::read_to_string(&path)
                    .expect("can't read file");

                let ext = path.extension().map(|ext| {
                    ext.to_str().unwrap().to_lowercase()
                });

                let lang = match ext.as_ref().map(|s| s.as_str()) {
                    Some("c") | Some("cc") | Some("h") | Some("cpp") | Some("hpp") |
                    Some("glsl") | Some("fs") | Some("vs")
                        => ProgramminLang::CLike,

                    Some("rs") => ProgramminLang::Rust,

                    _ => ProgramminLang::Other
                };

                let (loc, lloc) = count_loc(&src, lang);

                if args.display_per_file {
                    println!("phys {loc:5} log {lloc:5} file `{}`", path.to_str().unwrap());
                }

                loc_total += loc;
                lloc_total += lloc;
            }
        }
    }
    println!("total: physical {loc_total:4} logical {lloc_total:4}");

}

// physical, logical (c-like style)
fn count_loc(src: &str, lang: ProgramminLang) -> (u32, u32) {
    // TODO try partially parse langs? (like comments, strings, separate single-line expressions as different logic lines?)
    src.lines().fold((0, 0), |(loc, mut lloc), line| {
        lloc += is_sensetive_line(line, lang) as u32;
        (loc + 1, lloc)
    })
}

fn is_sensetive_line(line: &str, lang: ProgramminLang) -> bool {
    let line = line.trim();
    match lang {
        // For now stay C and Rust ~ same
        ProgramminLang::CLike |
        ProgramminLang::Rust => {
            !line.is_empty() &&
            !line.starts_with("//") &&
            !line.trim_matches(|ch| ['{', '(', '[', ']', ')', '}', ','].contains(&ch)).is_empty()
        }
        // oh no it doesn't work for whitespace-based langs
        ProgramminLang::Other => !line.is_empty()
    }
}