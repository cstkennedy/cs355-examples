use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::multispace0,
    combinator::{map, map_res},
    sequence::{delimited, preceded},
};

use crate::HtmlColor;

impl From<(u8, u8, u8)> for HtmlColor {
    fn from(color_components: (u8, u8, u8)) -> Self {
        let (red, green, blue) = color_components;
        HtmlColor { red, green, blue }
    }
}

pub struct HtmlColorParser;

impl HtmlColorParser {
    fn is_digit_for_base(base: u32) -> impl Fn(char) -> bool {
        move |c: char| c.is_digit(base)
    }

    fn from_base(base: u32) -> impl Fn(&str) -> Result<u8, std::num::ParseIntError> {
        move |input: &str| u8::from_str_radix(input, base)
    }

    fn hex_primary(input: &str) -> IResult<&str, u8> {
        map_res(
            take_while_m_n(2, 2, Self::is_digit_for_base(16)),
            Self::from_base(16),
        )
        .parse(input)
    }

    pub fn parse_hex_color(input: &str) -> IResult<&str, HtmlColor> {
        map(
            (
                preceded(tag("#"), Self::hex_primary),
                Self::hex_primary,
                Self::hex_primary,
            ),
            HtmlColor::from,
        )
        .parse(input)
    }

    fn decimal_primary(input: &str) -> IResult<&str, u8> {
        map_res(
            take_while_m_n(1, 3, Self::is_digit_for_base(10)),
            Self::from_base(10),
        )
        .parse(input)
    }

    pub fn parse_rgb_color(input: &str) -> IResult<&str, HtmlColor> {
        map(
            (
                preceded(tag("rgb("), Self::decimal_primary),
                preceded(
                    delimited(multispace0, tag(","), multispace0),
                    Self::decimal_primary,
                ),
                delimited(
                    delimited(multispace0, tag(","), multispace0),
                    Self::decimal_primary,
                    tag(")"),
                ),
            ),
            HtmlColor::from,
        )
        .parse(input)
    }

    pub fn parse_color(input: &str) -> IResult<&str, HtmlColor> {
        alt((
            Self::parse_hex_color,
            Self::parse_rgb_color,
        ))
        .parse(input)
    }
}
