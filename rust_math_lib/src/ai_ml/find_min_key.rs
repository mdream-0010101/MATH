use std::collections::HashMap;

/// Finds the key from 'stats' that corresponds to the minimum value in `merge`.
/// Treats keys not present in 'merge' as having an infinite value.
///
/// # arguments
///
/// * `stats` - A slice of string slices representing the key to be checked.
/// * `merge` - A reference to a MashMap where the keys are &str and the values are f64.
///
/// # Returns
///
/// The key corresponding to the minumum value, or None if `state` is empty.
pub fn find_min_keys<'a>(stats: &'a [&'a str], merge: &HashMap<&str, f64>) -> Option<&'a str> {
    stats
        .iter()
        .min_by(|&&x, &&y| {
            let x_val = merge.get(x).unwrap_or(&f64::INFINITY);
            let y_val = merge.get(y).unwrap_or(&f64::INFINITY);
            x_val.partial_cmp(&y_val).unwrap()
        })
        .copied() // to change fom &&str to &str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_key() {
        let merge = HashMap::from([("a", 1.0), ("b", 2.0)]);

        let stats = ["a", "b", "c", "d"];

        let min_key = find_min_keys(&stats, &merge);

        assert_eq!(min_key, Some("a"));
    }
}
