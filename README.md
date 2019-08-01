# Reading from both stdin and tty in crossterm

Attempts to read piped input from stdin in an interactive terminal interface
using [crossterm](https://github.com/TimonPost/crossterm).

## crossterm v0.8

In crossterm 0.8, it's possible to read from standard input (e.g. data to
process) and still receive separate tty input events (e.g. keypresses for an
interactive terminal UI).

See [`crossterm-0.8.2`](./crossterm-0.8.2) for an example:

```bash
cd crossterm-0.8.2/
cargo build

for i in $(seq 1 100); do echo $i; sleep 1; done | ./target/debug/crossterm-stdin-test 2> err.log
```

This works because piped content (in this case, output from the shell's counting
loop) is read from `/dev/stdin`, while crossterm reads terminal input
(keypresses) from `/dev/tty`.

There are a few issues minor issues with this approach:
 * we can't read from rust's usual `std::io::stdin` as it EOFs when crossterm
   enables raw mode and can't be reopened; the manual read from the `/dev/stdin`
   file works around this

   This hack should work on all UNIXes and has been tested on Linux and macOS.
 * if no content is piped in (i.e. just running
   `./target/debug/crossterm-stdin-test`), both `/dev/stdin` and `/dev/tty` seem
   to be the same file and the two readers fight over reading each character.

   TUI apps can work around this by checking if stdin is a tty using
   [`atty`](https://crates.io/crates/atty), e.g. `atty::is(Stream::Stdin)`, and
   leaving `/dev/stdin` closed if so.
 * Windows doesn't seem to have any issues reading from rust's normal stdin so
   this hack isn't needed

## crossterm v0.10

crossterm's API changed slightly since v0.8, but an equivalent program now
results in an error:

```bash
cd crossterm-0.10.1/
cargo build

for i in $(seq 1 100); do echo $i; sleep 1; done | ./target/debug/crossterm-stdin-test 2> err.log
```

The program will exit immediately and err.log will contain:
```
error opening raw mode: Os { code: 25, kind: Other, message: "Inappropriate ioctl for device" }
```

### Minimal reproduction

The [minimal reproduction](./crossterm-0.10.1-minimal) just tries to enable raw
mode. If you pipe content in, it prints the same error:

```bash
cd crossterm-0.10.1-minimal/

cargo build

echo '' | ./target/debug/crossterm-stdin-test
```

...fails and outputs the same message
