#[macro_use]
extern crate clap;
extern crate csv;
extern crate pty;
extern crate pty_shell;

use clap::{Arg, App};
use pty_shell::{PtyShell, PtyCallback};
use std::io::Write;
use std::time::Duration;

fn main() {
    let matches = App::new("replay")
        .version(crate_version!())
        .arg(Arg::with_name("input")
            .index(1)
            .help("Input file. Defaults to `recorded_input`."))
        .arg(Arg::with_name("keep")
            .short("k")
            .long("keep")
            .help("Keep the session alive after replay."))
        .get_matches();

    let filename = matches.value_of("input").unwrap_or("recorded_input");
    let keep = matches.is_present("keep");

    let mut reader = csv::Reader::from_file(filename)
        .expect(&format!("Could not find input file: `{}`", filename))
        .has_headers(false)
        .delimiter(b'\t')
        .flexible(true);

    let child = pty::fork().unwrap();
    child.exec(std::env::var("SHELL").unwrap_or(String::from("bash")))
        .unwrap();
    child.proxy(PtyCallback::new().build()).unwrap();
    // Child stops here. Now it's all the parent.

    print!("Starting replay.\r\n");
    let mut current = 0f64;
    for record in reader.decode() {
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
                child.pty().unwrap().write_all(&bytes).unwrap();
            }
            child.pty().unwrap().flush().unwrap();
            std::io::stdout().flush().unwrap();
        }
    }


    child.wait().unwrap();

    print!("Session replay complete.\r\n");
}
