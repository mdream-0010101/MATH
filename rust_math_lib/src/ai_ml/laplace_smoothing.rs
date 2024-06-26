use std::collections::HashMap;

/// Represents a bag of marbles with different colos.
/// the bag is modeled using HashMap where keys are color names (as strings)
/// and values are counts of how many marbles of that color are present.
struct MarbleBag {
    marbles: HashMap<String, u32>,
    total_marbles: u32,
}

impl MarbleBag {
    /// Construct a new `MarbleBag` from a given HashMap of marbles.
    ///
    /// # Arguments
    ///
    /// * `marbles` - A Hashmap were the key is a string representing the marble's color, and the
    /// value is the u32 representing the count of marbles of that color.
    ///
    /// # Returns
    ///
    /// A new `MarbleBag` instance.
    fn new(marbles: HashMap<String, u32>) -> Self {
        let total_marbles = marbles.values().sum();
        MarbleBag {
            marbles,
            total_marbles,
        }
    }

    fn probability_of_color(&self, color: &str) -> f64 {
        match self.marbles.get(color) {
            Some(&color_count) => {
                let total_count = self.total_marbles as f64 + self.marbles.len() as f64;
                (color_count as f64 + 1.0) / total_count
            }
            None => 1.0 / (self.total_marbles as f64 + self.marbles.len() as f64),
        }
    }
}

// Unit tests for our MarbleBag struct an methods
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probability_of_color() {
        // Setup: Intialize a MarbleBag with a new color
        let mut marbles = HashMap::new();
        marbles.insert("blue".to_string(), 50);
        marbles.insert("green".to_string(), 30);
        marbles.insert("red".to_string(), 20);
        // Note: no yellow marbles are added to the setup, this represents a unseen color.
        let bag = MarbleBag::new(marbles);

        // Calculates the probability of drawing a blue marble from a sunseen yellow marble.
        let blue_prob = bag.probability_of_color("blue");
        let yellow_prob = bag.probability_of_color("yellow");

        // Verify: Chec that the probabilities match the expected values after applying laplace
        // smoothing.
        assert_eq!(blue_prob, 51.0 / 104.0);
        assert_eq!(yellow_prob, 1.0 / 104.0);
    }
}
