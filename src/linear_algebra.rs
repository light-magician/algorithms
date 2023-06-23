
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