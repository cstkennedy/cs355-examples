use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    character::complete::multispace0,
    sequence::delimited,
    branch::alt
};

use crate::HtmlColor;

pub struct HtmlColorParser;

impl HtmlColorParser {
    // Source: https://github.com/rust-bakery/nom/tree/main?tab=readme-ov-file#example
    fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
        u8::from_str_radix(input, 16)
    }

    // Source: https://github.com/rust-bakery/nom/tree/main?tab=readme-ov-file#example
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
        let (input, (_, red, green, blue)) = (
            tag("#"),
            HtmlColorParser::hex_primary,
            HtmlColorParser::hex_primary,
            HtmlColorParser::hex_primary,
        )
            .parse(input)?;

        Ok((input, HtmlColor { red, green, blue }))
    }

    fn from_decimal(input: &str) -> Result<u8, std::num::ParseIntError> {
        u8::from_str_radix(input, 10)
    }

    fn is_decimal_digit(c: char) -> bool {
        c.is_digit(10)
    }

    fn decimal_primary(input: &str) -> IResult<&str, u8> {
        map_res(
            take_while_m_n(1, 3, HtmlColorParser::is_decimal_digit),
            HtmlColorParser::from_decimal,
        )
        .parse(input)
    }

    pub fn parse_rgb_color(input: &str) -> IResult<&str, HtmlColor> {
        let (input, (_, red, _, green, _, blue, _)) = (
            tag("rgb("),
            HtmlColorParser::decimal_primary,
            delimited(multispace0, tag(","), multispace0),
            HtmlColorParser::decimal_primary,
            delimited(multispace0, tag(","), multispace0),
            HtmlColorParser::decimal_primary,
            tag(")")
        )
            .parse(input)?;

        Ok((input, HtmlColor { red, green, blue }))
    }

    pub fn parse_color(input: &str) -> IResult<&str, HtmlColor> {
        alt((
            HtmlColorParser::parse_hex_color,
            HtmlColorParser::parse_rgb_color,
        )).parse(input)
    }
}
