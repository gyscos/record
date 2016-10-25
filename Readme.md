# `record`

Command-line utility to record a terminal session.

# Installation

`cargo install record`

# Example

```
$ record
Starting a recording session (saving input to `recorded_input`).
Press Ctrl-D or `exit` to stop.
$ ls
Dektop/ Mail/ Music/ Documents/ Pictures/
$ top
$ ^D
Input saved as `recorded_input`.
$ cat recorded_input
3.595   'l'
3.774   's'
4.878   '\r'
6.592   't'
6.878   'o'
6.927   'p'
8.001   '\r'
9.098   '\u{1b}'    'O' 'B'
9.333   '\u{1b}'    'O' 'B'
9.584   '\u{1b}'    'O' 'B'
9.874   '\u{1b}'    'O' 'A'
10.110  '\u{1b}'    'O' 'A'
10.380  '\u{1b}'    'O' 'C'
10.896  '\u{1b}'    'O' 'D'
11.624  'q'
12.727  '\u{4}'
$ replay
Starting replay.
$ ls
Dektop/ Mail/ Music/ Documents/ Pictures/
$ top
$ ^D
Session replay complete.
$
```

Each line is a tab-separated list of values.
The first value is the time in seconds since the beginning of the session.
The other values are the byte received at that time.
