use crate::three_min;

/// Example:
/// word: "back"
/// other: "bag"
/// matrix:
///   _ b a g
/// _ 0 1 2 3
/// b 1
/// a 2
/// c 3
/// k 4
pub fn check(word: &str, other: &str) -> u8 {
    let word_len = word.chars().count();
    let other_len = other.chars().count();
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; other_len + 1]; word_len + 1];

    for i in 0..=word_len {
        matrix[i][0] = i as u8;
    }
    for j in 0..=other_len {
        matrix[0][j] = j as u8;
    }

    for i in 1..=word_len {
        let word_ch = word.chars().nth(i - 1).unwrap();
        for j in 1..=other_len {
            let other_ch = other.chars().nth(j - 1).unwrap();

            if word_ch == other_ch {
                matrix[i][j] = matrix[i - 1][j - 1];
            } else {
                matrix[i][j] = 1 + three_min(
                    matrix[i - 1][j - 1],
                    matrix[i][j - 1],
                    matrix[i - 1][j],
                );
            }
        }
    }

    return matrix[word_len][other_len];
}


#[test]
fn check_test() {
    assert_eq!(check("bag", "back"), 2);
    assert_eq!(check("boats", "float"), 3);
}
