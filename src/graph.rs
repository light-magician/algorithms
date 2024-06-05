

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

use std::{collections::{HashSet, VecDeque}, usize};

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
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    /*
    its of charaters that need to be ints probably
    we need a set of tuples that we have seen before
    we need a deque that we can contribute to
    pass the grid as s ref

    seen is a set of tuples of i32 cooridinates
    queue will be contained in a helper function
        if we find an island 

    so how do we want to break this down
    double loop
    make a tuple
    has that tuple been checked before ?
    if not is that tuple land

    check directions ... a list of next directions

    making the visited a usize makes checking that its out of bounds too hard 
    check in bounds and is island
     */
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut island_count: i32 = 0;
    // x y cooridnates
    let x = grid.len();
    let y = grid[0].len();
    // grid loop
    for i in 0..x {
        for j in 0..y {
            let curr = (i as i32, j as i32);
            // you have to borrow values to check their in collections
            if !visited.contains(&curr) { 
                if is_land(&curr, &grid) {
                    island_count += 1;
                    visited.insert(curr);
                    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
                    q.push_back(curr);
                    visit_land(&mut visited, &grid, &mut q);
                } else {
                    visited.insert(curr);
                }

            }
        }
    }

    return island_count;
}

fn is_land(coordinate: &(i32, i32), grid: &Vec<Vec<char>>) -> bool {
    return grid[coordinate.0 as usize][coordinate.1 as usize] == '1';
}

fn visit_land(visited: &mut HashSet<(i32, i32)>, grid: &Vec<Vec<char>>, q: &mut VecDeque<(i32, i32)>) {
    // the goal is to visit the other related land on the grid in the 4 cardinal directions
    let search_directions = vec![
        (-1, 0), (1, 0), (0, 1), (0, -1)
    ];
    // x y cooridnates
    let x = grid.len() as i32;
    let y = grid[0].len() as i32;

    while !q.is_empty() {
        let curr = q.pop_front().unwrap();
        // check for land in cardinal directions around curr
        for direction in &search_directions {
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

fn is_inbounds(check: &(i32, i32), x: i32, y: i32) -> bool {
    // majorly important greater than or equal to zero, non-negative or >= size is illegal
    return (check.0 >= 0 && check.0 < x) && (check.1 >= 0 && check.1 < y);
}

#[cfg(test)]
mod graph_tests {
    use crate::graph::num_islands;

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

}