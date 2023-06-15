/*
graph algorithms
 */
mod graph {
    use rand::Rng;

    fn generate_valid_grid(w: usize, h: usize) -> Vec<Vec<i8>> {
        let mut rng = rand::thread_rng();
        // 2d array of random 0s and 1s
        let mut grid = vec![vec![0; w as usize]; h as usize];
        for i in 0..h {
            for j in 0..w {
                grid[i][j] = rng.gen_range(0..=1);
            }
        }
        let start = (0, 0);
        let end = (h - 1, w - 1);
        // set valid start and end
        grid[start.0][start.1] = 0;
        grid[end.0][end.1] = 0;
        // Generate random path
        let mut current = start;
        while current != end {
            let next = get_random_neighbor(current, w, h, &grid, &mut rng);
            grid[next.0][next.1] = 0;
            current = next;
        }

        grid
    }

    fn get_random_neighbor(
        position: (usize, usize),
        width: usize,
        height: usize,
        grid: &Vec<Vec<i8>>,
        rng: &mut impl Rng,
    ) -> (usize, usize) {
        //TODO: this can go on forever
        // implement validating which neighbors have been visited
        // implement a left, down, right, up function
        // and diagonals
        let mut neighbors = vec![];
    
        let x = position.0;
        let y = position.1;
    
        // Check top neighbor
        if y > 0 && grid[y - 1][x] == 0 {
            neighbors.push((x, y - 1));
        }
    
        // Check right neighbor
        if x < width - 1 && grid[y][x + 1] == 0 {
            neighbors.push((x + 1, y));
        }
    
        // Check bottom neighbor
        if y < height - 1 && grid[y + 1][x] == 0 {
            neighbors.push((x, y + 1));
        }
    
        // Check left neighbor
        if x > 0 && grid[y][x - 1] == 0 {
            neighbors.push((x - 1, y));
        }
    
        if neighbors.is_empty() {
            // No available neighbors, return current position
            position
        } else {
            // Return a random neighbor
            let neighbors_len = neighbors.len();
            neighbors[rng.gen_range(0..neighbors_len)]
            // neighbors.choose(rng).unwrap();
        }
    }


    #[cfg(test)]
    mod graph_test {
        use super::*;

        #[test]
        fn test_generate_grid() {
            let grid = generate_valid_grid(10, 10);
            print_grid(&grid);
        }

        fn print_grid(grid: &Vec<Vec<i8>>) {
            for row in grid {
                println!("{:?}", row);
            }
        }
    }

}