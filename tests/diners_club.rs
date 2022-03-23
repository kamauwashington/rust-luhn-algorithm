use luhn::test;

#[test]
fn should_pass_diners_club_1() {
    assert_eq!(test("30569309025904"), true)
}

#[test]
fn should_pass_diners_club_2() {
    assert_eq!(test("38520000023237"), true)
}

#[test]
fn should_fail() {
    assert_eq!(test("385202200023237"), false)
}
