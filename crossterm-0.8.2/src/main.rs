use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use crossterm::{
    Crossterm, TerminalInput, InputEvent, Screen, KeyEvent, ClearType
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

    let alt = match Screen::default().enable_alternate_modes(true) {
        Ok(alt) => alt,
        Err(e) => {
            eprintln!("error opening alternate mode: {:?}", e);
            return;
        }
    };

    let crossterm = Crossterm::from_screen(&alt.screen);
    let cursor = crossterm.cursor();
    let terminal = crossterm.terminal();

    let input = TerminalInput::from_output(&alt.screen.stdout);
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
