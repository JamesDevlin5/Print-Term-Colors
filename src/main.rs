use termion::{color, style};
fn main() {
    print!("{}   ", color::Bg(color::Black));
    print!("{}   ", color::Bg(color::Red));
    print!("{}   ", color::Bg(color::Green));
    print!("{}   ", color::Bg(color::Yellow));
    print!("{}   ", color::Bg(color::Blue));
    print!("{}   ", color::Bg(color::Magenta));
    print!("{}   ", color::Bg(color::Cyan));
    print!("{}   ", color::Bg(color::White));
    println!();
    print!("{}   ", color::Bg(color::LightBlack));
    print!("{}   ", color::Bg(color::LightRed));
    print!("{}   ", color::Bg(color::LightGreen));
    print!("{}   ", color::Bg(color::LightYellow));
    print!("{}   ", color::Bg(color::LightBlue));
    print!("{}   ", color::Bg(color::LightMagenta));
    print!("{}   ", color::Bg(color::LightCyan));
    print!("{}   ", color::Bg(color::LightWhite));
    println!("{}", style::Reset);
}
