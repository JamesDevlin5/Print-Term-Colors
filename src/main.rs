use termion::{color, style};

const BLOCK_CHARS: &str = "   ";

/// Gets a single background color along a group of three space characters.
/// This will effectively display a block of empty space that is colored by the supplied color.
/// Returns: A string, formatted with the color provided as the background of the block character
/// string.
fn get_color(c: &dyn color::Color) -> String {
    format!("{}{}", color::Bg(c), BLOCK_CHARS)
}

/// Gets a single line in accordance with the colors provided.
/// This function will ensure proper formatting of the line printed, included prefixed padding, printing of each color in the collection properly, and cleaning up after the line if finished.
/// Returns: A string, formatted with each color provided coloring the background of the block
/// characters.
fn get_line(cvec: &Vec<&dyn color::Color>) -> String {
    let mut s = String::with_capacity(cvec.len() * BLOCK_CHARS.len());
    for c in cvec {
        s.push_str(get_color(c).as_str());
    }
    s
}

fn pr_line(l: String) {
    println!("{}{}{}", BLOCK_CHARS, l, style::Reset);
}

fn main() {
    // A collection of all the regular colors
    let colors: Vec<&dyn color::Color> = vec![
        &color::Black,
        &color::Red,
        &color::Green,
        &color::Yellow,
        &color::Blue,
        &color::Magenta,
        &color::Cyan,
        &color::White,
    ];

    // A collection of all the light colors
    let lcolors: Vec<&dyn color::Color> = vec![
        &color::LightBlack,
        &color::LightRed,
        &color::LightGreen,
        &color::LightYellow,
        &color::LightBlue,
        &color::LightMagenta,
        &color::LightCyan,
        &color::LightWhite,
    ];

    pr_line(get_line(&colors));
    pr_line(get_line(&lcolors));
}
