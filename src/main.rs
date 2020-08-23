use termion::{color, style};

/// Prints a single background color along a group of three space characters.
/// This will effectively display a block of empty space that is colored by the supplied color.
fn pr_color(c: &dyn color::Color) {
    print!("{}   ", color::Bg(c));
}

/// Prints a single line in accordance with the colors provided.
/// This function will ensure proper formatting of the line printed, included prefixed padding, printing of each color in the collection properly, and cleaning up after the line if finished.
fn pr_line(cvec: &Vec<&dyn color::Color>) {
    print!("   ");
    for c in cvec {
        pr_color(c);
    }
    println!("{}", style::Reset);
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

    pr_line(&colors);
    pr_line(&lcolors);
}
