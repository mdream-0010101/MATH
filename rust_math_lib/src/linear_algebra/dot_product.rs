pub fn dot_product(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
    if a.len() != b.len() {
        Err("Vectors must be of the same length");
    } else {
        Ok(a.iter().zip(b.iter()).map(|(x, y)| x * y).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product_success() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let result = dot_product(&a, &b);

        assert_eq!(result, Ok(32.0));
    }

    #[test]
    fn test_dot_product_different_lengths() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let result = dot_product(&a, &b);

        assert!(result.is_err());
    }
}
