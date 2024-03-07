use std::collections::HashMap;

// function to simulate the Map step in MapReduce
fn map(document: String) -> HashMap<String, i32> {
    // Tokenize the document into words.
    let words = document.split_whitespace();
    let mut histogram = HashMap::new();

    // Count occurances of each words
    for word in words {
        *histogram.entry(word.to_string()).or_insert(0) += 1;
    }

    histogram
}

// function to simulate the Reduce step in MapReduce
fn reduce(term: String, values: Vec<i32>) -> i32 {
    // sum all counts for the term
    values.into_iter().sum()
}

// the `reduce` function adapted to include a mutable reference
fn reduce_with_mutable_reference(values: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in values.iter() {
        sum += value;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_single_word() {
        let document = "hello";
        let result = map(document);

        assert_eq!(result.get("hello"), Some(&1));
    }

    #[test]
    fn map_multiple_words() {
        let document = "hello world hello";
        let result = map(document);
        assert_eq!(result.get("hello"), Some(&2));
        assert_eq!(result.get("world"), Some(&1));
    }

    #[test]
    fn test_reduce_single_count() {
        let term = "hello";
        let value = vec![1, 2, 3];
        let result = reduce(term, values);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_reduce_no_counts() {
        let term = "world";
        let values: Vec<i32> = vec![];
        let result = reduce(term, values);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_reduce_with_mutable_reference() {
        let mut values = vec![1, 2, 3, 4, 5];
        let sum = reduce(&mut values);

        assert_eq!(sum, 15);
    }
}
