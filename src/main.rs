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

/// Formats the vector of colors provided, creating a string that is prefixed with an unformatted
/// block of characters, followed by a series of changes in background color, based on the colors in
/// the vector, and a grouping of block characters. Finally, the terminal styling will be reset to
/// ensure no issues after.
fn final_format(colors: Vec<&dyn color::Color>) -> String {
    format!("{}{}{}", BLOCK_CHARS, get_line(&colors), style::Reset)
}

/// Prints the vector of colors provided, as a single formatted line of block characters.
fn pr_line(colors: Vec<&dyn color::Color>) {
    println!("{}", final_format(colors));
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

    pr_line(colors);
    pr_line(lcolors);
}
