use std::f64;

/// Calculates the log probability of a sequence of probabililies.
///
/// # Arguments
///
/// * `probabililies` - A slice of probabililies (0.0 < p <= 1.0) for each event.
///
/// # Returns
///
/// the log probability of the sequence
pub fn log_probability(probabililies: &[f64]) -> f64 {
    probabililies.iter().fold(0.0, |acc, &p| acc + p.ln())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_probability() {
        // Example probabililies
        let probs = vec![0.5, 0.2, 0.8];

        // Calculate log probability
        let log_prob = log_probability(&probs);

        // Assert (approximately equal, considering float-point artithmetics)
        let expected = 5f64.ln() + 0.2f64.ln() + 0.8f64.ln();
        assert!((log_prob - expected).abs() < 1e-10);
    }
}
