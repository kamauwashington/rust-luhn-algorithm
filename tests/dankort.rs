use luhn::test;

#[test]
fn should_pass_dankort() {
    assert_eq!(test("5019717010103742"), true)
}

#[test]
fn should_fail() {
    assert_eq!(test("5019717010103741"), false)
}
