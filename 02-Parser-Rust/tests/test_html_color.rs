use hamcrest2::prelude::*;
use rstest::*;

use html_color::prelude::*;
use html_color::consts::*;

#[rstest]
pub fn test_default() {
    let color = HtmlColor::default();

    assert_that!(color.red, is(equal_to(0)));
    assert_that!(color.green, is(equal_to(0)));
    assert_that!(color.blue, is(equal_to(0)));

    assert_that!(color.to_string(), is(equal_to("#000000")));
}

#[rstest]
pub fn test_constructor() {
    let color = HtmlColor::new(11, 5, 255);

    assert_that!(color.red, is(equal_to(11)));
    assert_that!(color.green, is(equal_to(5)));
    assert_that!(color.blue, is(equal_to(255)));

    assert_that!(color.to_string(), is(equal_to("#0B05FF")));
}

#[rstest]
pub fn test_ord() {
    assert_that!(RED, is(greater_than(GREEN)));
    assert_that!(GREEN, is(greater_than(BLUE)));
    assert_that!(RED, is(greater_than(BLUE)));
}

#[rstest]
pub fn test_eq() {
    let color = HtmlColor::new(11, 5, 255);

    assert_that!(&color, not(equal_to(&RED)));
    assert_that!(&color, not(equal_to(&GREEN)));
    assert_that!(&color, not(equal_to(&BLUE)));
}

#[rstest]
pub fn test_hash() {
    todo!()
}

#[rstest]
#[case::black("#000000", &BLACK)]
#[case::white("#FFFFFF", &WHITE)]
#[case::red("#FF0000", &RED)]
#[case::green("#00FF00", &GREEN)]
#[case::blue("#0000FF", &BLUE)]
pub fn test_parse_hex(#[case] hex_string: &str, #[case] expected_color: &HtmlColor) {
    let result = HtmlColorParser::parse_hex_color(&hex_string);

    assert_that!(&result, ok());

    let (_, color) = result.unwrap();
    assert_that!(&color, is(equal_to(expected_color)));
}

#[rstest]
#[case::black("rgb(0, 0, 0)", &BLACK)]
#[case::white("rgb(255, 255, 255)", &WHITE)]
#[case::red("rgb(255, 0, 0)", &RED)]
#[case::green("rgb(0, 255, 0)", &GREEN)]
#[case::blue("rgb(0, 0, 255)", &BLUE)]
pub fn test_parse_rgb(#[case] rgb_string: &str, #[case] expected_color: &HtmlColor) {
    let result = HtmlColorParser::parse_rgb_color(&rgb_string);

    assert_that!(&result, ok());

    let (_, color) = result.unwrap();
    assert_that!(&color, is(equal_to(expected_color)));
}

#[rstest]
#[case::black_hex("#000000", &BLACK)]
#[case::white_hex("#FFFFFF", &WHITE)]
#[case::red_hex("#FF0000", &RED)]
#[case::green_hex("#00FF00", &GREEN)]
#[case::blue_hex("#0000FF", &BLUE)]
#[case::black_rgb("rgb(0, 0, 0)", &BLACK)]
#[case::white_rgb("rgb(255, 255, 255)", &WHITE)]
#[case::red_rgb("rgb(255, 0, 0)", &RED)]
#[case::green_rgb("rgb(0, 255, 0)", &GREEN)]
#[case::blue_rgb("rgb(0, 0, 255)", &BLUE)]
pub fn test_parse(#[case] rgb_string: &str, #[case] expected_color: &HtmlColor) {
    let result = HtmlColorParser::parse_color(&rgb_string);

    assert_that!(&result, ok());

    let (_, color) = result.unwrap();
    assert_that!(&color, is(equal_to(expected_color)));
}
