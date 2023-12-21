use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start(grid: Vec<Vec<char>>) -> [usize;2] {
    for (y, row) in grid.iter().enumerate() {
        for x in 0..row.len() {
            if grid[y][x] == 'S' {
                return [y,x];
            }
        }
    }
    panic!("start wasn't found");
}

fn tranverse(
    start: &[usize; 2],
    grid: &Vec<Vec<char>>,
    current: [usize; 2],
    hashset: &mut HashSet<[usize; 2]>,
    limit: i32,
) {
    if manhetan_distance(*start, current) > limit || grid[current[0]][current[1]] == '#' {
        return;
    }
    hashset.insert(current);
    if current[0] + 1 < grid.len() {
        tranverse(start, grid, [current[0] + 1, current[1]], hashset, limit);
    }
    if current[0] > 0 {
        tranverse(start, grid, [current[0] - 1, current[1]], hashset, limit);
    }
    if current[1] > 0 {
        tranverse(start, grid, [current[0], current[1] - 1], hashset, limit);
    }
    if current[1] + 1 < grid[0].len() {
        tranverse(start, grid, [current[0], current[1] + 1], hashset, limit);
    }
}

fn manhetan_distance(a: [usize; 2], b: [usize; 2]) -> i32 {
    (a[0] as i32 - b[0] as i32).abs() + (a[1] as i32 - b[1] as i32).abs()
}

fn solve(input: &str) -> usize {
    let gird = parse(&input);
    let 
}



#[cfg(test)]
mod tests {
    use super::*;
    pub const TEST_INPUT: &str = 
"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    #[test]
    fn it_works() {
        let result = 2 + 2;
        let sol
        assert_eq!(result, 4);
    }
}
