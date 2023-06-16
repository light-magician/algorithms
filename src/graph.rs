/*
graph algorithms
 */
mod graph {
    use rand::{Rng, thread_rng, seq::SliceRandom};

    const moves: Vec<(i32,i32)> = vec![
        (-1, -1), // up left
        (-1, 0), // up
        (-1, 1), // up right
        (0, -1), // left
        (0, 1), // right
        (1, -1), // down left
        (1, 0), // down
        (1, 1), // down right
    ];

    fn generate_valid_grid(w: i32, h: i32) -> Vec<Vec<i32>> {
        let mut rng = rand::thread_rng();
        // 2d array of random 0s and 1s
        let mut grid = vec![vec![0; w as usize]; h as usize];
        for i in 0..h {
            for j in 0..w {
                grid[i as usize][j as usize] = rng.gen_range(0..=1);
            }
        }
        let start = (0, 0);
        let end = (h - 1, w - 1);
        // Generate random path
        generate_valid_path(&mut grid, start, end);

        grid
    }

    fn generate_valid_path(grid: &mut Vec<Vec<i32>>, start: (i32, i32), end: (i32, i32)) {
        /*
        make start and end valid
        shuffle the list of next moves
        if a next move is valid and helps minimize distance, move there
        if none are alter a move that minimizes distance
         */
        // set valid start and end
        grid[start.0 as usize][start.1 as usize] = 0;
        grid[end.0 as usize][end.1 as usize] = 0;

        let mut rng = thread_rng();
        let mut current: (i32, i32) = start;
        /*
        we want to know if it is in bounds and it decreases the euclidian distance
         */
        while current != end {
            moves.shuffle(&mut rng);
            for (x_curr, y_curr) in &moves {
                let next: (i32, i32) = (current.0 as i32 + x_curr, current.1 as i32 + y_curr);
                // if random diretion is closer and is an obsticle, flip it
                if is_in_bounds(grid, next) && is_closer(current, next, end) && is_obsticle(grid, next) {
                        grid[next.0 as usize][next.1 as usize] = 0;
                        current = next;
                        break;
                // if its not an obsticle move forward
                } else if is_in_bounds(grid, next) && is_closer(current, next, end) {
                    current = next;
                    break;
                }
            }
        }
    }

    fn is_obsticle(grid: &Vec<Vec<i32>>, next: (i32, i32)) -> bool {
        grid[next.0 as usize][next.1 as usize] == 1
    }

    fn is_closer(current:(i32, i32), next: (i32, i32), end: (i32, i32)) -> bool {
        let next_dist = euclidian_distance(next, end);
        let curr_dist = euclidian_distance(current, end);
        next_dist < curr_dist
    }

    fn euclidian_distance(next: (i32, i32), end: (i32, i32)) -> i32 {
        let x = (end.0 - next.0 as i32).pow(2);
        let y = (end.1 - next.1 as i32).pow(2);
        let distance = (x + y) as f64;
        distance.sqrt().floor() as i32
    }


    fn is_in_bounds(grid: &Vec<Vec<i32>>, next: (i32, i32)) -> bool {
        let h = grid.len() as i32;
        let w = grid[0].len() as i32;
        next.0 >= 0 && next.0 < h && next.1 >= 0 && next.1 < w
    }

    mod traversals {
        /*
        TODO: implement bfs, dfs, dijkstra, a*
        write the algorithms and make sure they are both tested and benchmarked
            and able to 
         */

        use std::collections::{VecDeque, HashSet};

        use super::{is_obsticle, is_in_bounds, moves};

        fn bfs(grid: &Vec<Vec<i32>>) {

        }

        fn dfs(grid: &Vec<Vec<i32>>, start: (i32, i32), end: (i32, i32)) -> bool {
            let rows = grid.len();
            let cols = grid[0].len();
            let mut q = VecDeque::new();
            let mut visited: HashSet<(i32, i32)> = HashSet::new();

            q.push_back(start);
            visited.insert(start);
            while !q.is_empty() {
                let curr = q.pop_front().unwrap();
                // we have reached the end
                if curr == end {
                    return true;
                }
                let (x, y) = (curr.0, curr.1);
                for (x_curr, y_curr) in &moves {
                    let next: (i32, i32) = (x + x_curr, y + y_curr);
                    if is_in_bounds(grid, next) && !visited.contains(&next) && !is_obsticle(grid, next) {
                        q.push_back(next);
                        visited.insert(next);
                    }
                }
            }
            // unable to reach the end
            false
        }

        fn dijkstra(grid: &Vec<Vec<i32>>) {

        }

        fn a_star(grid: &Vec<Vec<i32>>) {

        }

    }

    #[cfg(test)]
    mod graph_test {
        use super::*;

        #[test]
        fn test_generate_grid() {
            let grid = generate_valid_grid(100, 100);
            print_grid(&grid);
        }

        fn print_grid(grid: &Vec<Vec<i32>>) {
            for row in grid {
                println!("{:?}", row);
            }
        }
    }

}