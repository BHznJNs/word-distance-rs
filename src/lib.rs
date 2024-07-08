pub mod levenshtein;
pub mod wagner_fischer;

fn three_min<T: Ord>(a: T, b: T, c: T) -> T {
    if a < b && a < c {
        return a;
    }
    if b < c {
        return b;
    } else {
        return c;
    }
}

#[test]
fn three_min_test() {
    assert_eq!(three_min(1, 2, 3), 1);
    assert_eq!(three_min(3, 2, 1), 1);
}
