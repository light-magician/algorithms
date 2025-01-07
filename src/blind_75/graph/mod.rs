#[cfg(test)]

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    use std::collections::HashSet;
    let land: char = '1';
    let water: char = '0';
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
}

mod graph_tests {

    use super::*;

    const GRID: [[char; 5]; 4] = [
        ['1', '1', '1', '1', '0'],
        ['1', '1', '0', '1', '0'],
        ['1', '1', '0', '0', '0'],
        ['0', '0', '0', '0', '0'],
    ];

    #[test]
    fn test_num_islands() {
        // need to dfs an island structure and mark those
        // coordinates as seen
        // if coord is potential island -> not seen
        //      search it out
        //  do not add seen land to search
        // when an island is found dfs or bfs out the rest of it
    }
}
