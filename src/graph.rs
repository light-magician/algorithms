// Given an m x n 2D binary grid grid which represents a map of
// '1's (land) and '0's (water), return the number of islands.
// An island is surrounded by water and is formed by connecting
// adjacent lands horizontally or vertically. You may assume all
//  four edges of the grid are all surrounded by water.
// Example 1:
// Input: grid = [
//   ["1","1","1","1","0"],
//   ["1","1","0","1","0"],
//   ["1","1","0","0","0"],
//   ["0","0","0","0","0"]
// ]
// Output: 1
// Example 2:
// Input: grid = [
//   ["1","1","0","0","0"],
//   ["1","1","0","0","0"],
//   ["0","0","1","0","0"],
//   ["0","0","0","1","1"]
// ]
// Output: 3
// Constraints:
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 300
// grid[i][j] is '0' or '1'.
//
// some are dfs
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    use std::{
        collections::{HashSet, VecDeque},
        usize,
    };

    fn is_land(coordinate: &(i32, i32), grid: &Vec<Vec<char>>) -> bool {
        return grid[coordinate.0 as usize][coordinate.1 as usize] == '1';
    }
    fn is_inbounds(check: &(i32, i32), x: i32, y: i32) -> bool {
        return (check.0 >= 0 && check.0 < x) && (check.1 >= 0 && check.1 < y);
    }
    fn visit_land(
        visited: &mut HashSet<(i32, i32)>,
        grid: &Vec<Vec<char>>,
        q: &mut VecDeque<(i32, i32)>,
        search_directions: &Vec<(i32, i32)>,
        x: i32,
        y: i32,
    ) {
        // breadth first search
        while !q.is_empty() {
            let curr = q.pop_front().unwrap();
            // check for land in cardinal directions around curr
            for direction in search_directions {
                let check = (curr.0 + direction.0, curr.1 + direction.1);
                // out of bounds skip
                if !is_inbounds(&check, x, y) {
                    continue;
                }
                if !visited.contains(&check) {
                    if is_land(&check, &grid) {
                        q.push_back(check);
                    }
                    visited.insert(check);
                }
            }
        }
    }

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut island_count: i32 = 0;
    // x y cooridnates
    let x = grid.len();
    let y = grid[0].len();

    // search directions
    let search_directions = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    // grid loop
    for i in 0..x {
        for j in 0..y {
            let curr = (i as i32, j as i32);
            // you have to borrow values to check their in collections
            if is_land(&curr, &grid) && !visited.contains(&curr) {
                island_count += 1;
                visited.insert(curr);
                let mut q: VecDeque<(i32, i32)> = VecDeque::new();
                q.push_back(curr);
                visit_land(
                    &mut visited,
                    &grid,
                    &mut q,
                    &search_directions,
                    x as i32,
                    y as i32,
                );
            } else {
                visited.insert(curr);
            }
        }
    }
    island_count
}

pub fn num_islands_no_set(grid: Vec<Vec<char>>) -> i32 {
    use std::collections::VecDeque;
    // breadth first search
    fn bfs(grid: &mut Vec<Vec<char>>, directions: &Vec<(i32, i32)>, coord: (i32, i32)) {
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back(coord);
        while !q.is_empty() {
            let coord = q.pop_front().unwrap();
            for d in directions {
                let diff = (coord.0 + d.0, coord.1 + d.1);
                if diff.0 >= 0
                    && diff.0 < grid.len() as i32
                    && diff.1 >= 0
                    && diff.1 < grid[0].len() as i32
                    && grid[diff.0 as usize][diff.1 as usize] == '1'
                {
                    grid[diff.0 as usize][diff.1 as usize] = '0';
                    q.push_back(diff);
                }
            }
        }
    }
    // dont have to check if its
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut islands = 0;
    let mut grid = grid;
    // search
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' {
                islands += 1;
                grid[i][j] = '0';
                bfs(&mut grid, &directions, (i as i32, j as i32));
            }
        }
    }
    islands
}
#[cfg(test)]
mod graph_tests {
    use crate::graph::{num_islands, num_islands_no_set};

    #[test]
    fn num_islands_test() {
        let grid: Vec<Vec<char>> = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(1, num_islands(grid));
    }

    #[test]
    fn num_islands_no_set_test() {
        let grid: Vec<Vec<char>> = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(1, num_islands_no_set(grid));
    }
}
