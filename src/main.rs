#[macro_use]
extern crate clap;
extern crate time;
extern crate pty;
extern crate pty_shell;

use pty_shell::{PtyShell, PtyCallback};


use std::io::Write;
use clap::{Arg, App};


fn main() {
    let matches = App::new("record")
        .about("\
Starts a recording session.
All input will be recorded to the given file.

The resulting file will consist in a number of lines, each prefixed with the
time in seconds since the beginning of the session, and a list of bytes sent.")
        .version(crate_version!())
        .arg(Arg::with_name("output")
            .index(1)
            .help("Output file. Defaults to `recorded_input`."))
        .get_matches();

    let filename = matches.value_of("output").unwrap_or("recorded_input");

    let mut output = std::fs::File::create(filename).unwrap();

    println!("\
Starting a recording session (saving input to `{}`).
Press Ctrl-D or `exit` to stop.",
             filename);
    let child = pty::fork().unwrap();

    let start = time::precise_time_s();
    child.exec(std::env::var("SHELL").unwrap_or(String::from("bash"))).unwrap();
    child.proxy(PtyCallback::new()
            .input(move |input| {
                write!(&mut output, "{:.3}", time::precise_time_s() - start).unwrap();
                for &byte in input {
                    write!(&mut output, " {:?}", byte as char).unwrap();
                }
                writeln!(&mut output, "").unwrap();
            })
            .build())
        .unwrap();

    child.wait().unwrap();

    print!("Input saved as `{}`.\r\n", filename);
}
