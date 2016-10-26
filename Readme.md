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
0.932	'l'
1.103	's'
1.599	'\r'
2.477	't'
2.631	'o'
2.687	'p'
3.579	'\r'
4.882	'\u{1b}'	'O'	'B'
5.137	'\u{1b}'	'O'	'B'
5.879	'\u{1b}'	'O'	'A'
6.125	'\u{1b}'	'O'	'A'
7.959	'q'
9.723	'\u{4}'
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
