use termion::{color, style};

pub const BLOCK_CHARS: &str = "   ";

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_color() {
        {
            let mut s = String::new();
            s.push_str(format!("{}", color::Bg(color::Red)).as_str());
            s.push_str(BLOCK_CHARS);
            assert_eq!(get_color(&color::Red), s);
        }
        {
            let mut s = String::new();
            s.push_str(format!("{}", color::Bg(color::Blue)).as_str());
            s.push_str(BLOCK_CHARS);
            assert_eq!(get_color(&color::Blue), s);
        }
        {
            let mut s = String::new();
            s.push_str(format!("{}", color::Bg(color::Green)).as_str());
            s.push_str(BLOCK_CHARS);
            assert_eq!(get_color(&color::Green), s);
        }
    }
    #[test]
    fn test_get_line() {
        let mut s = String::new();
        s.push_str(format!("{}", color::Bg(color::Red)).as_str());
        s.push_str(BLOCK_CHARS);
        assert_eq!(get_line(&vec![&color::Red]), s);
        s.push_str(format!("{}", color::Bg(color::Yellow)).as_str());
        s.push_str(BLOCK_CHARS);
        assert_eq!(get_line(&vec![&color::Red, &color::Yellow]), s);
        s.push_str(format!("{}", color::Bg(color::Cyan)).as_str());
        s.push_str(BLOCK_CHARS);
        assert_eq!(
            get_line(&vec![&color::Red, &color::Yellow, &color::Cyan]),
            s
        );
    }

    #[test]
    fn test_final_format() {
        let mut s = String::new();
        s.push_str(BLOCK_CHARS);

        s.push_str(format!("{}", color::Bg(color::Red)).as_str());
        s.push_str(BLOCK_CHARS);

        let mut s1 = s.clone();
        s1.push_str(format!("{}", style::Reset).as_str());
        assert_eq!(final_format(vec![&color::Red]), s1);

        s.push_str(format!("{}", color::Bg(color::Yellow)).as_str());
        s.push_str(BLOCK_CHARS);

        let mut s2 = s.clone();
        s2.push_str(format!("{}", style::Reset).as_str());
        assert_eq!(
            format!("{}", final_format(vec![&color::Red, &color::Yellow])),
            s2
        );

        s.push_str(format!("{}", color::Bg(color::Cyan)).as_str());
        s.push_str(BLOCK_CHARS);

        let mut s3 = s.clone();
        s3.push_str(format!("{}", style::Reset).as_str());
        assert_eq!(
            format!(
                "{}",
                final_format(vec![&color::Red, &color::Yellow, &color::Cyan])
            ),
            s3
        );
    }
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
