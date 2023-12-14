mod input;

enum Direction {
    North,
    West,
    South,
    East,
}

fn main() {
    let result = solve(&input::INPUT);
    println!("{result}");
}

fn solve(input: &str) -> usize {
    let map = parse(input);
    let tilted_map = tilt_north(&map);
    calculate_weights(&tilted_map)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_ranges(map: &Vec<Vec<char>>, index: usize, direction: Direction) -> Vec<[usize; 2]> {
    match direction {
        Direction::North => get_ranges_north(map, index),
        Direction::South => get_ranges_south(map, index),
        Direction::West => get_ranges_west(map, index),
        Direction::East => get_ranges_east(map, index),
    }
}

fn get_ranges_north(map: &Vec<Vec<char>>, collum: usize) -> Vec<[usize; 2]> {
    let mut ranges = vec![];
    let mut current_range = vec![];
    for y in (0..map.len()).rev() {
        if map[y][collum] == '#' || y == 0 || y == map.len() - 1 {
            current_range.push(y);
        }
        if current_range.len() == 2 {
            ranges.push([current_range[0], current_range[1]]);
            current_range = vec![current_range[1]];
        }
    }
    ranges
}

fn get_ranges_south(map: &Vec<Vec<char>>, collum: usize) -> Vec<[usize; 2]> {
    let mut ranges = vec![];
    let mut current_range = vec![];
    for y in (0..map.len()).rev() {
        if map[y][collum] == '#' || y == 0 || y == map.len() - 1 {
            current_range.push(y);
        }
        if current_range.len() == 2 {
            ranges.push([current_range[0], current_range[1]]);
            current_range = vec![current_range[1]];
        }
    }
    ranges
}

fn get_ranges_east(map: &Vec<Vec<char>>, row: usize) -> Vec<[usize; 2]> {
    let mut ranges = vec![];
    let mut current_range = vec![];
    for x in (0..map[0].len()).rev() {
        if map[row][x] == '#' || x == 0 || x == map[0].len() - 1 {
            current_range.push(x);
        }
        if current_range.len() == 2 {
            ranges.push([current_range[0], current_range[1]]);
            current_range = vec![current_range[1]];
        }
    }
    ranges
}

fn get_ranges_west(map: &Vec<Vec<char>>, row: usize) -> Vec<[usize; 2]> {
    let mut ranges = vec![];
    let mut current_range = vec![];
    for x in 0..map[0].len() {
        if map[row][x] == '#' || x == 0 || x == map[0].len() - 1 {
            current_range.push(x);
        }
        if current_range.len() == 2 {
            ranges.push([current_range[0], current_range[1]]);
            current_range = vec![current_range[1]];
        }
    }
    ranges
}

fn get_boulder_between_range(
    map: &Vec<Vec<char>>,
    collum: usize,
    range: [usize; 2],
    direction: Direction,
) -> Vec<usize> {
    let mut result = vec![];
    for y in range[1]..=range[0] {
        if map[y][collum] == 'O' {
            result.push(y);
        }
    }
    result
}

fn generate_empty_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    map.iter()
        .map(|row| {
            row.iter()
                .map(|square| if square == &'O' { '.' } else { *square })
                .collect()
        })
        .collect()
}

fn tilt_vertically(map: &Vec<Vec<char>>, direction: Direction) -> Vec<Vec<char>> {
    let mut map_clone = generate_empty_map(map);
    for collum in 0..map[0].len() {
        let ranges = get_ranges(map, collum, direction);
        for range in ranges {
            let boulders = get_boulder_between_range(map, collum, range);
            let starting = if range[1] == 0 && map_clone[range[1]][collum] == '.' {
                range[1]
            } else {
                range[1] + 1
            };
            for (i, _boulder) in boulders.iter().enumerate() {
                map_clone[starting + i][collum] = 'O';
            }
        }
    }
    map_clone
}

fn tilt_horizontally(map: &Vec<Vec<char>>, direction: Direction) -> Vec<Vec<char>> {
    let mut map_clone = generate_empty_map(map);
    for row in 0..map[0] {
        let ranges = get_ranges(map, row, direction);
        for range in ranges {
            let boulders = get_boulder_between_range(map, collum, range, direction);
            let starting = if range[1] == 0 && map_clone[range[1]][collum] == '.' {
                range[1]
            } else {
                range[1] + 1
            };
            for (i, _boulder) in boulders.iter().enumerate() {
                map_clone[starting + i][collum] = 'O';
            }
        }
    }
    map_clone
}

fn calculate_weights(map: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for (y, row) in map.iter().enumerate() {
        for square in row {
            if square == &'O' {
                sum += map.len() - y;
            }
        }
    }
    sum
}

fn print_map(map: &Vec<Vec<char>>) {
    let result: String = map
        .iter()
        .map(|row| row.iter().collect::<String>() + "\n")
        .collect();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    pub const TEST_RESULT: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    pub const TEST_RESULT_PLUS_1: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#O...";

    pub const PART_OF_INPUT: &str = ".#....#.O##O...
......O........
.##O.O...O.....
.O.O.#...##.O.#
#..#..#..#.#.O.
...............
..O..OO.O...##.
....##.#..#...O
O..O.###...OO..
.....#......O#O
.......#....#..
.O..O.....O#..O
........O..OO.#";
    #[test]
    fn range_test_1() {
        let map = parse(TEST_INPUT);
        let result = get_ranges(&map, 0, Direction::North);
        assert_eq!(
            result,
            vec![[map.len() - 1, map.len() - 2], [map.len() - 2, 0]]
        );
    }
    #[test]
    fn boulder_indexes_test() {
        let map = parse(TEST_INPUT);
        let ranges = get_ranges(&map, 0, Direction::North);
        let boulders = get_boulder_between_range(&map, 0, ranges[1]);

        assert_eq!(boulders.len(), 4);
    }
    #[test]
    fn north_tilt_test() {
        let map = parse(TEST_INPUT);
        let expected = parse(TEST_RESULT);
        let result = tilt_north(&map);
        assert_eq!(result, expected);
    }
    #[test]
    fn north_tilt_test2() {
        let map = parse(PART_OF_INPUT);
        let result = tilt_north(&map);
        assert_eq!(result[0][result[0].len() - 1], '.');
    }
    #[test]
    fn north_tilt_test3() {
        let input = "#
O
#";
        let map = parse(input);
        let result = tilt_north(&map);
        println!("map is {:?}", result);
        assert_eq!(result[1][0], 'O');
    }
    #[test]
    fn north_tilt_test4() {
        let input = "#
O
O
#";
        let map = parse(input);
        let result = tilt_north(&map);
        println!("map is {:?}", result);
        assert_eq!(result[2][0], 'O');
        assert_eq!(result[1][0], 'O');
    }
    #[test]
    fn part_1_test() {
        let result = solve(TEST_INPUT);
        assert_eq!(result, 136);
    }
    #[test]
    fn part_1_test_plus_1() {
        let result = solve(TEST_RESULT_PLUS_1);
        assert_eq!(result, 137);
    }
}
