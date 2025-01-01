/// This is matrix multiplication done in Rust
///
/// CPU Optimizations:
///     block matrix multiplication to improve cache utilization
///     parallel processing using rayon
///     flat array representation for better mem access patterns
///     manual loop unrolling for instruction level parallelism
/// GPU Optimizations ...
use std::sync::Arc;

/// lets write a way to multiply matricies that is more efficient
/// lets multiply like normal but with flattening
/// the second part of the Result return is what is used for a generic panic!
pub fn mat_mul_naive(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, &'static str> {
    let a_rows: usize = a.len();
    let a_cols: usize = a[0].len();
    let b_rows: usize = a.len();
    let b_cols: usize = a[0].len();
    // note that this makes a_cols and b_rows the shared dimension
    validate_dimensions(a_cols, b_rows);
    // result has a_rows and b_cols, both not shared dimensions
    let mut result: Vec<Vec<f64>> = result_matrix(a_rows, b_cols);
    // its an n^3 operation
    // result at ij = a[row][shared dimension] * b[shared_dimension][b_cols]
    // we want to multiplied their shared dimension index for each
    // column of b and that for each row of a
    for i in 0..a_rows {
        for j in 0..b_cols {
            for k in 0..a_cols {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    Ok(result)
}

fn validate_dimensions(a_cols: usize, b_rows: usize) {
    if a_cols != b_rows {
        panic!("dimensions don't add up");
    }
}

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize
}

struct FlatMatrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize
}


/// Flat matrix implementation
struct MatrixBlock<'a> {
    data: &'a [f64],
    start_row: usize,
    end_row: usize,
    cols: usize,
}

struct MatrixConfig {
    block_size: usize,
    parallelization_threshold: usize,
}

pub fn mat_mul_inline(
    a: &Vec<Vec<f64>>,
    b: &Vec<Vec<f64>>,
    config: Option<MatrixConfig>,
) -> Result<Vec<Vec<f64>>, &'static str> {
    // set config as a var config and unwrap it
    // validate matrix dimensions
    let a_rows: usize = a.len();
    let a_cols: usize = a[0].len();
    let b_rows: usize = a.len();
    let b_cols: usize = a[0].len();
    
    let result = result_matrix(a_rows, b_cols);

    Ok(result)
}

fn flatten(arr: Vec<Vec<f64>>) -> Vec<f64> {
    arr.iter().flat_map(|row| row.iter().cloned()).collect()
}

fn result_matrix(a_rows: usize, b_cols: usize) -> Vec<Vec<f64>> {
    return vec![vec![0.0; b_cols]; a_rows];
}

fn should_use_parallel_processing(size: usize, config: &MatrixConfig) -> bool {
    size >= config.parallelization_threshold
}

#[cfg(test)]
mod linear_algebra_test {
    use super::*;

    #[test]
    fn test_square_matrix() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let expected = vec![vec![19.0, 22.0], vec![43.0, 50.0]];
        assert_eq!(mat_mul_naive(&a, &b).unwrap(), expected);
    }

    #[test]
    fn test_non_square_valid() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let b = vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]];
        let expected = vec![vec![58.0, 64.0], vec![139.0, 154.0]];
        assert_eq!(mat_mul_naive(&a, &b).unwrap(), expected);
    }

    #[test]
    fn test_identity_matrix() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        assert_eq!(mat_mul_naive(&a, &b).unwrap(), b);
    }

    #[test]
    fn test_zero_matrix() {
        let a = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let expected = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
        assert_eq!(mat_mul_naive(&a, &b).unwrap(), expected);
    }

    #[test]
    fn test_large_matrix() {
        let size = 100;
        let a = vec![vec![1.0; size]; size];
        let b = vec![vec![1.0; size]; size];
        let result = mat_mul_naive(&a, &b).unwrap();
        assert_eq!(result[0][0], size as f64);
    }

    //TODO: test the matrix cannot be multiplied panic
}
