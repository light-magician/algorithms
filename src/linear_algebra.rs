/// This is matrix multiplication done in Rust
/// 
/// CPU Optimizations:
///     block matrix multiplication to improve cache utilization
///     parallel processing using rayon
///     flat array representation for better mem access patterns
///     manual loop unrolling for instruction level parallelism
/// GPU Optimizations ...

/// here 'a is a borrow ref that means that 
struct MatrixBlock<'a> {
    data: &'a [f64],
    start_row: usize,
    end_row: usize,
    cols: usize,
}

struct Matrix_Config {
    block_size: usize,
    parallelization_threshold: usize,
}

impl Default for Matrix_Config {
    fn default() -> Self {
        // default block size for cache optimization
        // minimim size before using parallel processing
        Self {
            block_size: 32,
            parallelization_threshold: 64
        }
    }
}

/// its really a matrix that you want to multiply
/// a alters row, b alters column
pub fn mat_mul_naive(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, MatrixMultiplyError> {
    let a_rows: usize = a.len();
    let a_cols: usize = a[0].len();
    let b_rows: usize = b.len();
    let b_cols: usize = b[0].len();

    // flattening, now access with i * number of cols + j rather than @i @j
    let a_flat: Vec<f64> = a.iter().flat_map(|row| row.iter().cloned()).collect();
    let b_flat: Vec<f64> = a.iter().flat_map(|row| row.iter().cloned()).collect();

    // validate dimensions
    if !can_mul(a_cols, b_rows) {
        return Err(MatrixMultiplyError::DimensionMismatch {
            a_rows,
            a_cols,
            b_rows,
            b_cols,
        });
    }

    // preallocate result array of correct size
    // result will be of a_rows x b_cols
    let mut result = vec![vec![0.0; b_cols]; a_rows];

    for i in 0..a_rows {
        for j in 0..b_cols {
            for k in 0..a_cols {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    Ok(result)
}


#[derive(Debug)]
pub enum MatrixMultiplyError {
    DimensionMismatch {
        a_rows: usize,
        a_cols: usize,
        b_rows: usize,
        b_cols: usize,
    }
}

// You can implement Display if you want the same formatting:
impl std::fmt::Display for MatrixMultiplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MatrixMultiplyError::DimensionMismatch { a_rows, a_cols, b_rows, b_cols } => {
                write!(f, "matrix A dimensions ({}, {}) \nmatrix B dimensions ({}, {})", 
                    a_rows, a_cols, b_rows, b_cols)
            }
        }
    }
}

#[cfg(test)]
mod linear_algebra_test {
    use super::*;

    #[test]
    fn test_square_matrix() {
        let a = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0]
        ];
        let b = vec![
            vec![5.0, 6.0],
            vec![7.0, 8.0]
        ];
        let expected = vec![
            vec![19.0, 22.0],
            vec![43.0, 50.0]
        ];
        assert_eq!(mat_mul_naive(&a, &b).unwrap(), expected);
    }

    #[test]
    fn test_non_square_valid() {
        let a = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0]
        ];
        let b = vec![
            vec![7.0, 8.0],
            vec![9.0, 10.0],
            vec![11.0, 12.0]
        ];
        let expected = vec![
            vec![58.0, 64.0],
            vec![139.0, 154.0]
        ];
        assert_eq!(mat_mul_naive(&a, &b).unwrap(), expected);
    }

    #[test]
    fn test_identity_matrix() {
        let a = vec![
            vec![1.0, 0.0],
            vec![0.0, 1.0]
        ];
        let b = vec![
            vec![5.0, 6.0],
            vec![7.0, 8.0]
        ];
        assert_eq!(mat_mul_naive(&a, &b).unwrap(), b);
    }

    #[test]
    fn test_zero_matrix() {
        let a = vec![
            vec![0.0, 0.0],
            vec![0.0, 0.0]
        ];
        let b = vec![
            vec![5.0, 6.0],
            vec![7.0, 8.0]
        ];
        let expected = vec![
            vec![0.0, 0.0],
            vec![0.0, 0.0]
        ];
        assert_eq!(mat_mul_naive(&a, &b).unwrap(), expected);
    }

    #[test]
    fn test_dimension_mismatch() {
        let a = vec![
            vec![1.0, 2.0]
        ];
        let b = vec![
            vec![3.0],
            vec![4.0],
            vec![5.0]
        ];
        assert!(matches!(
            mat_mul_naive(&a, &b),
            Err(MatrixMultiplyError::DimensionMismatch { .. })
        ));
    }

    #[test]
    fn test_large_matrix() {
        let size = 100;
        let a = vec![vec![1.0; size]; size];
        let b = vec![vec![1.0; size]; size];
        let result = mat_mul_naive(&a, &b).unwrap();
        assert_eq!(result[0][0], size as f64);
    }
}

