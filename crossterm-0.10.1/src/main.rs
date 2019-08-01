use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use crossterm::{
    Crossterm, AlternateScreen, InputEvent, RawScreen, KeyEvent, ClearType
};

fn main() {
    let (tx, rx) = channel();

    // spawn a thread to read from /dev/stdin
    // entering raw mode in crossterm seems to break std::io::stdin
    thread::spawn(move || {
        let file = File::open("/dev/stdin").unwrap();
        for line in BufReader::new(file).lines() {
            tx.send(line.unwrap()).ok();
        }
    });

    let _screen = match RawScreen::into_raw_mode() {
        Ok(screen) => screen,
        Err(e) => {
            eprintln!("error opening raw mode: {:?}", e);
            return;
        }
    };

    // don't enable raw mode here via to_alternate(true) as it can mask the
    // source of errors (opening alternate screen vs opening raw)
    if let Err(e) = AlternateScreen::to_alternate(false) {
        eprintln!("error opening alternate mode: {:?}", e);
        return;
    };

    let crossterm = Crossterm::new();
    let cursor = crossterm.cursor();
    let terminal = crossterm.terminal();
    let input = crossterm.input();
    let mut input_events = input.read_async();

    terminal.clear(ClearType::All).ok();

    let mut lines: usize = 0;
    let mut last_line = None;

    'outer: loop {
        for line in rx.try_iter() {
            lines += 1;
            last_line = Some(line);
        }

        cursor.goto(0, 0).ok();
        terminal.write(format!("read lines: {}", lines)).ok();

        cursor.goto(0, 1).ok();
        terminal.clear(ClearType::CurrentLine).ok();

        if let Some(line) = &last_line {
            terminal.write(format!("last line: {}", line)).ok();
        }

        while let Some(event) = input_events.next() {
            match event {
                InputEvent::Keyboard(KeyEvent::Char('q')) => break 'outer,
                _ => {
                    cursor.goto(0, 3).ok();
                    terminal.clear(ClearType::CurrentLine).ok();
                    terminal.write(format!("event: {:?}", event)).ok();
                }
            };
        }

        thread::sleep(Duration::from_micros(1000));
    }
}
