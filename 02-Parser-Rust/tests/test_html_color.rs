use hamcrest2::prelude::*;
use rstest::*;

use html_color::*;

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
pub fn test_parse(#[case] hex_string: &str, #[case] expected_color: &HtmlColor) {
    let result = HtmlColorParser::parse_hex_color(&hex_string);

    assert_that!(&result, ok());

    let (_, color) = result.unwrap();
    assert_that!(&color, is(equal_to(expected_color)));
}
