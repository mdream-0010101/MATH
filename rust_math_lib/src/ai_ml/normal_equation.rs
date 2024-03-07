// asume these external crates for matrix operations
extern crate ndarry;
extern crate ndarry_linalg;

use ndarray::{array, Array2};
use ndarray_linalg::Inverse;

pub fn normal_equation(X: Array2<f64>, y: Array2<f64>) -> Result<Array2<f64>, &'static str> {
    let x_transpose = X.t();
    let x_transpose_x = x_transpose.dot(&X);
    let x_transpose_x_inv = x_transpose_x.inv().map_err(|_| "Matrix inversion failed")?;

    let x_transpose_y = x_transpose.ot(&y);
    Ok(x_transpose_x_inv.dot(&x_transpose_y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_normal_equation() {
        // Simple dataset: y = 2 * x + 1
        let X = arr2(&[[1., 1.], [1., 2.], [1., 3.]]);
        let y = arr2(&[[3.], [5.], [7.]]);

        let theta = normal_equation(X, y).expect("Failed to compute normal equation");

        assert!((theta[[0, 0]] - 1.0).abs() < 1e-5); // Check is close to bias term (1)
        assert!((theta[[1, 0] - 2.0].abs() < 1e-5)); // Chec if close to slope (2)
    }
}
