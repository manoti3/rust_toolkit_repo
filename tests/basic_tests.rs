#[test]
fn test_addition() {
    assert_eq!(2 + 3, 5);
}

#[test]
fn test_string_reverse() {
    let s = "rust";
    let reversed: String = s.chars().rev().collect();
    assert_eq!(reversed, "tsur");
}
