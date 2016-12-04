#[macro_use]
extern crate clap;
extern crate time;
extern crate pty;
extern crate pty_shell;

use clap::{Arg, App};
use pty_shell::{PtyShell, PtyCallback};


use std::io::Write;


fn main() {
    let matches = App::new("record")
        .about("\
Starts a recording session.
All input will be recorded to \
                the given file.

The resulting file will consist in a \
                number of lines, each prefixed with the
time in seconds \
                since the beginning of the session, and a list of bytes \
                sent.")
        .version(crate_version!())
        .arg(Arg::with_name("output")
            .index(1)
            .help("Output file. Defaults to `recorded_input`."))
        .get_matches();

    let filename = matches.value_of("output").unwrap_or("recorded_input");

    println!("\
Starting a recording session (saving input to `{}`).
Press Ctrl-D or `exit` to stop.",
             filename);

    let child = pty::fork().unwrap();
    child.exec(std::env::var("SHELL").unwrap_or(String::from("bash")))
        .unwrap();
    // Child stops here. Now it's all the parent.

    let mut output = std::fs::File::create(filename).unwrap();
    let start = time::precise_time_s();
    child.proxy(PtyCallback::new()
            .input(move |input| {
                write!(&mut output, "{:.3}", time::precise_time_s() - start)
                    .unwrap();
                for &byte in input {
                    if byte < 128 {
                        write!(&mut output, "\t{:?}", byte as char).unwrap();
                    } else {
                        write!(&mut output, "\t'\\u{{{:x}}}'", byte).unwrap();
                    }
                }
                writeln!(&mut output, "").unwrap();
            })
            .build())
        .unwrap();

    child.wait().unwrap();

    print!("Input saved as `{}`.\r\n", filename);
}
