use crossterm::RawScreen;

fn main() {
    let _screen = match RawScreen::into_raw_mode() {
        Ok(screen) => screen,
        Err(e) => {
            eprintln!("error opening raw mode: {:?}", e);
            return;
        }
    };

    println!("hello world");
}
