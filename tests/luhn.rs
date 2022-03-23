use luhn::test;

#[test]
fn should_fail_on_empty() {
    assert_eq!(test(""), false)
}

#[test]
fn should_fail_on_no_digits() {
    assert_eq!(test("  hello I am not valid CC input   "), false)
}

#[test]
fn should_fail_if_less_than_20_digits() {
    assert_eq!(test("1"), false)
}

#[test]
fn should_succeed_on_2_digits_with_valid_checkdigit() {
    assert_eq!(test("91"), true)
}

#[test]
fn should_succeed_on_formatted_cc() {
    assert_eq!(test("5455 3307 6000 0018"), true)
}