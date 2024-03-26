use std::collections::HashMap;
/// Check Permutation Given two strings, write a method to decide if one is permutation of the
///

fn is_permutation(str1: &str, str2: &str) -> bool {
    // if lengths are different, they cannot be permutations;
    if str1.len() != str2.len() {
        return false;
    }

    let mut char_count: HashMap<char, usize> = HashMap::new();

    // count charaters in str1;
    for c in str1.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    // decrement counts for characters in str2;
    for c in str2.chars() {
        match char_count.get_mut(&c) {
            Some(count) => {
                if *count == 0 {
                    return false;
                }
                *count -= 1;
            }
            None => return false,
        }
    }

    char_count.values().all(|&count| count == 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_permutation() {
        assert_eq!(is_permutation("abc", "abd"), false);
        assert_eq!(is_permutation("abc", "bca"), true);
    }
}
