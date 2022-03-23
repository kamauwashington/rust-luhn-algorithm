use luhn::test;

#[test]
fn should_pass_jcb_1() {
    assert_eq!(test("3530111333300000"), true)
}

#[test]
fn should_pass_jcb_2() {
    assert_eq!(test("3566002020360505"), true)
}

#[test]
fn should_fail() {
    assert_eq!(test("3566002020250505"), false)
}
