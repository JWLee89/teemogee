use crate::string::reverse;

#[test]
fn reverse_test() {
    assert_eq!(reverse("str"), "rts");
    assert_eq!(reverse("string"), "gnirts");
    assert_eq!(reverse(""), "");
}