use std::collections::HashMap;

/// a simple bigram model that uses absolute discounting.
struct BigramModel {
    bigram_counts: HashMap<(String, String), u32>,
    total_bigrams: u32,
}

impl BigramModel {
    /// creates a new, empty bigram model.
    fn new() -> Self {
        BigramModel {
            bigram_counts: HashMap::new(),
            total_bigrams: 0,
        }
    }

    /// adds a bigram to the model or increments its count if it already exists
    fn add_bigram(&mut self, first_word: &str, second_word: &str) {
        let bigram = (first_word.to_string(), second_word.to_string());
        let count = self.bigram_counts.entry(bigram).or_insert(0);

        *count += 1;
        self.total_bigrams += 1;
    }

    /// Applies absolute discounting to the count of a specific bigram.
    /// Returns the discounted probablity of the bigram.
    fn discount_bigram(&self, first_word: &str, second_word: &str, discount_factor: f32) -> f32 {
        let bigram = (first_word.to_string(), second_word.to_string());
        let count = *self.bigram_counts.get(&bigram).unwrap_or(&0) as f32;
        let discounted_count = if count > 0.0 {
            count - discount_factor
        } else {
            0.0
        };
        discounted_count / self.total_bigrams as f32
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_discount_bigram() {
        let mut model = BigramModel::new();
        model.add_bigram("hello", "world");
        model.add_bigram("hello", "world");
        model.add_bigram("hello", "rust");

        // assuming a discount factor of 0.75 (from the book)
        let prob = model.discount_bigram("hello", "world", 0.75);
        assert!(prob > 0.0);
    }
}
