// function to simulate the Map step in MapReduce
fn map(document: String) -> Vec<(String, i32)> {
    // Tokenize the document into words.
    let words = document.split_whitespace();
    let mut histogram = vec![];

    // Count occurances of`` each words
    for word in words {
        if let Some(entry) = histogram.iter_mut().find(|(w, _)| *w == word) {
            entry.1 += 1;
        } else {
            histogram.push((word.to_string(), 1));
        }
    }

    histogram
}

// function to simulate the Reduce step in MapReduce
fn reduce(values: Vec<i32>) -> i32 {
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
        let document = "hello".to_string();
        let result = map(document);

        assert_eq!(format!("{:?}", result), "[(\"hello\", 1)]");
    }

    #[test]
    fn map_multiple_words() {
        let document = "hello world hello".to_string();
        let result = map(document);

        assert_eq!(format!("{:?}", result), "[(\"hello\", 2), (\"world\", 1)]")
    }

    #[test]
    fn test_reduce_single_count() {
        let values = vec![1, 2, 3];
        let result = reduce(values);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_reduce_no_counts() {
        let values: Vec<i32> = vec![];
        let result = reduce(values);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_reduce_with_mutable_reference() {
        let mut values = vec![1, 2, 3, 4, 5];
        let sum = reduce_with_mutable_reference(&mut values);

        assert_eq!(sum, 15);
    }
}
