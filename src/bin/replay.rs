use clap::Parser;
use pty_shell::{PtyCallback, PtyShell};
use std::io::Write;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input file. Defaults to `recorded_input`.
    input: Option<String>,

    #[arg(short, long)]
    keep: bool,
}

fn main() {
    let args = Args::parse();

    let filename = args.input.as_deref().unwrap_or("recorded_input");
    let keep = args.keep;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .flexible(true)
        .from_path(filename)
        .expect(&format!("Could not find input file: `{}`", filename));

    let child = pty::fork::Fork::from_ptmx().unwrap();
    child
        .exec(std::env::var("SHELL").unwrap_or(String::from("bash")))
        .unwrap();
    child.proxy(PtyCallback::new().build()).unwrap();
    // Child stops here. Now it's all the parent.

    print!("Starting replay.\r\n");
    let mut current = 0f64;
    for record in reader.deserialize() {
        let (time, record): (f64, Vec<String>) = record.unwrap();
        let duration_ms = (1000f64 * (time - current)) as u64;
        let duration = Duration::from_millis(duration_ms);
        std::thread::sleep(duration);
        current = time;
        for token in record {
            let token = &token[1..token.len() - 1];
            let bytes = if token.starts_with('\\') {
                vec![match token {
                    "\\\"" => b'"',
                    "\\'" => b'\'',
                    "\\r" => b'\r',
                    "\\t" => b'\t',
                    other if other.starts_with("\\x") => {
                        let n = &other[2..other.len() - 1];
                        u8::from_str_radix(n, 16).unwrap()
                    }
                    other if other.starts_with("\\u{") => {
                        let n = &other[3..other.len() - 1];
                        u8::from_str_radix(n, 16).unwrap()
                    }
                    other => panic!("Found token {:?}", other),
                }]
            } else {
                token.as_bytes().to_vec()
            };

            if !(keep && &bytes == &[4]) {
                child.is_parent().unwrap().write_all(&bytes).unwrap();
            }
            child.is_parent().unwrap().flush().unwrap();
            std::io::stdout().flush().unwrap();
        }
    }

    child.wait().unwrap();

    print!("Session replay complete.\r\n");
}
