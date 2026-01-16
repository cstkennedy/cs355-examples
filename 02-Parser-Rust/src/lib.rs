use std::fmt;

use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
};

/// An HTML Color is represented by a combination of red, green and blue. Each
/// component (red, green, and blue) must fall in the range [0, 255].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HtmlColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub const BLACK: HtmlColor = HtmlColor::new(0, 0, 0);
pub const WHITE: HtmlColor = HtmlColor::new(255, 255, 255);
pub const RED: HtmlColor = HtmlColor::new(255, 0, 0);
pub const GREEN: HtmlColor = HtmlColor::new(0, 255, 0);
pub const BLUE: HtmlColor = HtmlColor::new(0, 0, 255);

impl Default for HtmlColor {
    /// Construct an HTML Color with all
    /// attributes set to 0 (i.e., black, #000000)
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl HtmlColor {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl fmt::Display for HtmlColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

//------------------------------------------------------------------------------
// Source: https://github.com/rust-bakery/nom/tree/main?tab=readme-ov-file#example

pub struct HtmlColorParser;

impl HtmlColorParser {
    fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
        u8::from_str_radix(input, 16)
    }

    fn is_hex_digit(c: char) -> bool {
        c.is_digit(16)
    }

    fn hex_primary(input: &str) -> IResult<&str, u8> {
        map_res(
            take_while_m_n(2, 2, HtmlColorParser::is_hex_digit),
            HtmlColorParser::from_hex,
        )
        .parse(input)
    }

    pub fn parse_hex_color(input: &str) -> IResult<&str, HtmlColor> {
        let (input, _) = tag("#")(input)?;
        let (input, (red, green, blue)) = (
            HtmlColorParser::hex_primary,
            HtmlColorParser::hex_primary,
            HtmlColorParser::hex_primary,
        )
            .parse(input)?;

        Ok((input, HtmlColor { red, green, blue }))
    }
}
