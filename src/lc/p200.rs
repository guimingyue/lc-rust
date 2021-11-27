use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn num_islands_dfs(grid: Vec<Vec<char>>) -> i32 {
        fn dfs(map: &mut Vec<Vec<char>>, l: i32, c: i32) {
            if l < 0 || c < 0 || l >= map.len() as i32 || c >= map[0].len() as i32 || map[l as usize][c as usize] == '0' {
                return;
            }
            map[l as usize][c as usize] = '0';
            dfs(map, l+1, c);
            dfs(map, l-1, c);
            dfs(map, l, c+1);
            dfs(map, l, c-1);
        }
        let mut grid = grid;
        let mut num_islands = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    num_islands += 1;
                    dfs(&mut grid, i as i32, j as i32);
                }
            }
        }
        num_islands
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        fn bfs(map: &mut Vec<Vec<char>>, l: i32, c: i32) {
            if l < 0 || c < 0 || l >= map.len() as i32 || c >= map[0].len() as i32 || map[l as usize][c as usize] == '0' {
                return;
            }
            let mut queue = Vec::new();
            map[l as usize][c as usize] = '0';
            queue.push((l, c));
            while !queue.is_empty() {
                let (lv, cv) = queue.remove(0);
                if lv - 1 >= 0 && map[(lv-1) as usize][cv as usize] == '1' {
                    queue.push((lv-1, cv));
                    map[(lv-1) as usize][cv as usize] = '0'
                }
                if lv + 1 < map.len() as i32 && map[(lv+1) as usize][cv as usize] == '1' {
                    queue.push((lv+1, cv));
                    map[(lv+1) as usize][cv as usize] = '0'
                }
                if cv - 1 >= 0 && map[lv as usize][(cv-1) as usize] == '1' {
                    queue.push((lv, cv-1));
                    map[lv as usize][(cv-1) as usize] = '0'
                }
                if cv + 1 < map[0].len() as i32 && map[lv as usize][(cv+1) as usize] == '1' {
                    queue.push((lv, cv+1));
                    map[lv as usize][(cv+1) as usize] = '0'
                }
            }

        }

        let mut grid = grid;
        let mut num_islands = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    num_islands += 1;
                    bfs(&mut grid, i as i32, j as i32);
                }
            }
        }
        num_islands
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::num_islands(vec![vec!['1','1','1','1','0'],
                                        vec!['1','1','0','1','0'],
                                        vec!['1','1','0','0','0'],
                                        vec!['0','0','0','0','0']]));
    assert_eq!(3, Solution::num_islands(vec![vec!['1','1','0','0','0'],
                                             vec!['1','1','0','0','0'],
                                             vec!['0','0','1','0','0'],
                                             vec!['0','0','0','1','1']
    ]));
    assert_eq!(1, Solution::num_islands(vec![
        vec!['1','1','1','1','1','0','1','1','1','1','1','1','1','1','1','0','1','0','1','1'],
        vec!['0','1','1','1','1','1','1','1','1','1','1','1','1','0','1','1','1','1','1','0'],
        vec!['1','0','1','1','1','0','0','1','1','0','1','1','1','1','1','1','1','1','1','1'],
        vec!['1','1','1','1','0','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'],
        vec!['1','0','0','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'],
        vec!['1','0','1','1','1','1','1','1','0','1','1','1','0','1','1','1','0','1','1','1'],
        vec!['0','1','1','1','1','1','1','1','1','1','1','1','0','1','1','0','1','1','1','1'],
        vec!['1','1','1','1','1','1','1','1','1','1','1','1','0','1','1','1','1','0','1','1'],
        vec!['1','1','1','1','1','1','1','1','1','1','0','1','1','1','1','1','1','1','1','1'],
        vec!['1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'],
        vec!['0','1','1','1','1','1','1','1','0','1','1','1','1','1','1','1','1','1','1','1'],
        vec!['1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'],
        vec!['1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'],
        vec!['1','1','1','1','1','0','1','1','1','1','1','1','1','0','1','1','1','1','1','1'],
        vec!['1','0','1','1','1','1','1','0','1','1','1','0','1','1','1','1','0','1','1','1'],
        vec!['1','1','1','1','1','1','1','1','1','1','1','1','0','1','1','1','1','1','1','0'],
        vec!['1','1','1','1','1','1','1','1','1','1','1','1','1','0','1','1','1','1','0','0'],
        vec!['1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'],
        vec!['1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'],
        vec!['1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'],
    ]))
}