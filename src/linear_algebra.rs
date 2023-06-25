
/**
 * add matricies
 * multiply matricies
 * compute dot products
 * whats back prop?
 */

pub mod linear_algebra {

    // function that generates 2D matrix of random numbers from 1 to 10
    pub fn generate_matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
        let mut matrix: Vec<Vec<i32>> = Vec::new();
        for _ in 0..rows {
            let mut row: Vec<i32> = Vec::new();
            for _ in 0..cols {
                row.push(rand::random::<i32>() % 10);
            }
            matrix.push(row);
        }
        matrix
    }

    pub dot(matrix1: Vec<Vec<i32>>, matrix2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        /*
        3x2 * 2x3 = 3x3
        rows of matrix 1 must == cols of matrix 2
         */
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..matrix1.len() {
            let mut row: Vec<i32> = Vec::new();
            for j in 0..matrix2[0].len() {
                let mut sum = 0;
                for k in 0..matrix2.len() {
                    sum += matrix1[i][k] * matrix2[k][j];
                }
                row.push(sum);
            }
            result.push(row);
        }
        result
    }


    mod linear_algebra_test {
        use super::*;

        #[test]
        fn test_generate_matrix() {
            let matrix = generate_matrix(3, 3);
            assert_eq!(matrix.len(), 3);
            assert_eq!(matrix[0].len(), 3);
        }
    }
}