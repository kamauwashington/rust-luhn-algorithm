use luhn::test;

#[test]
fn should_pass_aus_bankcard() {
    assert_eq!(test("5610591081018250"), true)
}

#[test]
fn should_fail() {
    assert_eq!(test("5610581091018250"), false)
}
