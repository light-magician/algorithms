/// its really a matrix that you want to multiply
/// a alters row, b alters column
pub fn mat_mul_naive(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, MatrixMultiplyError> {
    // go down the row of a and multiply with the col of B
    // that will go inside i, j
    // check that the matrix can be multiplied
    // formulate the new matrix
    // find the mem address where THIS result is going
    // facilitate the multiplication
    let a_rows: usize = a.len();
    let a_cols: usize = a[0].len();
    let b_rows: usize = b.len();
    let b_cols: usize = b[0].len();

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
    // let mut a_row: usize = 0;
    // let mut a_col: usize = 0;
    // let mut b_row: usize = 0;
    // let mut b_col: usize = 0;

    for i in 0..a_rows {
        for j in 0..b_cols {
            for k in 0..a_cols {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    Ok(result)
}

fn can_mul(a_cols: usize, b_rows: usize) -> bool {
    a_cols == b_rows
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

