use luhn::test;

#[test]
fn should_pass_amex_1() {
    assert_eq!(test("378282246310005"), true)
}

#[test]
fn should_pass_amex_2() {
    assert_eq!(test("371449635398431"), true)
}

#[test]
fn should_pass_amex_corporate() {
    assert_eq!(test("378734493671000"), true)
}

#[test]
fn should_fail() {
    assert_eq!(test("3782 82246 310004"), false)
}
