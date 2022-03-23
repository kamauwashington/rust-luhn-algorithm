use luhn::test;

#[test]
fn should_pass_visa_1() {
    assert_eq!(test("4111111111111111"), true)
}

#[test]
fn should_pass_visa_2() {
    assert_eq!(test("4012888888881881"), true)
}

#[test]
fn should_pass_visa_3() {
    assert_eq!(test("4222222222222"), true)
}

#[test]
fn should_fail() {
    assert_eq!(test("4222322222222"), false)
}
