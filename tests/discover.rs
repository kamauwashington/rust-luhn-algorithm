use luhn::test;

#[test]
fn should_pass_discover_1() {
    assert_eq!(test("6011111111111117"), true)
}

#[test]
fn should_pass_discover_2() {
    assert_eq!(test("6011000990139424"), true)
}

#[test]
fn should_fail() {
    assert_eq!(test("6011000390134424"), false)
}
