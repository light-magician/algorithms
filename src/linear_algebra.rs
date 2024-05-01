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

    pub fn dot(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        /*
        3x2 * 2x3 = 3x3
        3x2 * 2x1 would give a 3x1 matrix
        cols of matrix 1 must == rows of matrix 2
         */
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..m1.len() {
            let mut row: Vec<i32> = Vec::new();
            for j in 0..m2[0].len() {
                let mut sum = 0;
                for k in 0..m2.len() {
                    sum += m1[i][k] * m2[k][j];
                }
                row.push(sum);
            }
            result.push(row);
        }
        result
    }

    pub fn can_multiply(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>) -> bool {
        // cols of matrix 1 must == rows of matrix 2
        return m1.len() == m2[0].len();
        // have to be sure that every row has the same number of corresponding columns
    }

    #[cfg(test)]
    mod linear_algebra_test {
        use super::*;

        #[test]
        fn test_generate_matrix() {
            let matrix = generate_matrix(3, 3);
            assert_eq!(matrix.len(), 3);
            assert_eq!(matrix[0].len(), 3);
        }

        #[test]
        fn test_can_multiply() {
            // obvious case
            let mut m1 = generate_matrix(3, 3);
            let mut m2 = generate_matrix(3, 3);
            assert_eq!(can_multiply(&m1, &m2), true);
            // 3x1 * 2x3 = 3x3
            m1 = generate_matrix(3, 1);
            m2 = generate_matrix(2, 3);
            assert_eq!(can_multiply(&m1, &m2), true);
            // 2x3 * 3x1 = false
            assert_eq!(can_multiply(&m2, &m1), false);
        }
    }
}