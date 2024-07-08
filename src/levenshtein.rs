use crate::three_min;

pub fn check(word: &str, other: &str) -> u8 {
    if word.is_empty() {
        return other.chars().count() as u8;
    }
    if other.is_empty() {
        return word.chars().count() as u8;
    }

    // first letter is same
    if word.chars().next() == other.chars().next() {
        return check(&word[1..], &other[1..]);
    }

    return 1 + three_min(
        check(&word[1..], &other[1..]),
        check(&word[1..], other),
        check(word, &other[1..]),
    );
}

#[test]
fn check_test() {
    assert_eq!(check("check", "cheat"), 2);
    assert_eq!(check("check", "checked"), 2);
    assert_eq!(check("hat", "can"), 2);
}
