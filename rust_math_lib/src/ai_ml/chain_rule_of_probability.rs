/// Implementing the chain rule:
/// P(A ∩ B ∩ C) = P(A) * P(B|A) * P(C|A ∩ B)

pub fn chain_rule(p_a: f64, p_b_given_a: f64, p_c_given_a_and_b: f64) -> f64 {
    p_a * p_b_given_a * p_c_given_a_and_b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_rule() {
        // define the input probablilities
        let p_a = 0.5; // Probabilitiy of A
        let p_b_given_a = 0.8; // Probabilitiy of B given A
        let p_c_given_a_and_b = 0.7; // Probabilitiy of C given A and B

        // Expected output
        let expected = 0.28; // Expected result of the chain rule calculation

        // Calculate the actual result rusing the chain_rule function
        let actual = chain_rule(p_a, p_b_given_a, p_c_given_a_and_b);

        assert_eq!(
            actual, expected,
            "The chain rule calculation did not producte the expected result."
        );
    }
}
