use crate::linear_algebra::dot_product::dot_product;

fn norm(v: &[f64]) -> f64 {
    v.iter().map(|&x| x * x).sum::<f64>().sqrt()
}

fn cauchy_schwarz_inequality(a: &[f64], b: &[f64]) -> bool {
    match dot_product(a, b) {
        Ok(dot_prod) => {
            let norms_prod = norm(a) * norm(b);
            dot_prod.abs() <= norms_prod
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear_algebra::dot_product::dot_product;

    #[test]
    fn test_dot_product() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        assert_eq!(dot_product(&a, &b).unwrap(), 32.0);
    }

    #[test]
    fn test_norm() {
        let v = [3.0, 4.0];
        assert_eq!(norm(&v), 5.0);
    }

    #[test]
    fn test_cauchy_schwarz_inequality() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];

        assert!(cauchy_schwarz_inequality(&a, &b));
    }
}
