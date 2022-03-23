use luhn::test;

#[test]
fn should_pass_mastercard_1() {
    assert_eq!(test("5555555555554444"), true)
}

#[test]
fn should_pass_mastercard_2() {
    assert_eq!(test("5105105105105100"), true)
}

#[test]
fn should_fail() {
    assert_eq!(test("5105105105203100"), false)
}
