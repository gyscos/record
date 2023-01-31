use pty_shell::{PtyCallback, PtyShell};

use clap::Parser;

use std::io::Write;

/// Starts a recording session.
/// All input will be recorded to the given file.
///
/// The resulting file will consist in a number of lines, each prefixed with
/// the time in seconds since the beginning of the session, and a list of
/// bytes sent.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Output file. Defaults to `recorded_input`.
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let filename = args.output.as_deref().unwrap_or("recorded_input");

    println!(
        "\
Starting a recording session (saving input to `{}`).
Press Ctrl-D or `exit` to stop.",
        filename
    );

    let child = pty::fork::Fork::from_ptmx().unwrap();
    child
        .exec(std::env::var("SHELL").unwrap_or(String::from("bash")))
        .unwrap();
    // Child stops here. Now it's all the parent.

    let mut output = std::fs::File::create(filename).unwrap();
    let start = time::Instant::now();
    child
        .proxy(
            PtyCallback::new()
                .input(move |input| {
                    write!(
                        &mut output,
                        "{:.3}",
                        start.elapsed().as_seconds_f32(),
                    )
                    .unwrap();
                    for &byte in input {
                        if byte < 128 {
                            write!(&mut output, "\t{:?}", byte as char)
                                .unwrap();
                        } else {
                            write!(&mut output, "\t'\\x{:x}'", byte).unwrap();
                        }
                    }
                    writeln!(&mut output, "").unwrap();
                })
                .build(),
        )
        .unwrap();

    child.wait().unwrap();

    print!("Input saved as `{}`.\r\n", filename);
}
