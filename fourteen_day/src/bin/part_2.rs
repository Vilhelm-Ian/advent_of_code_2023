mod input;
use std::collections::HashMap;
use std::collections::HashSet;

enum Direction {
    North,
    West,
    South,
    East,
}

fn main() {
    let result = solve(&input::INPUT, 1000000000);
    println!("{result}");
}

fn solve(input: &str, cycles: i32) -> usize {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut iterations = HashMap::new();
    let mut cycle = vec![];
    let mut cycle_start = false;
    let mut cycle_end = false;
    for _ in 0..cycles {
        map = tilt_cycle(&map.clone());
        println!("{:?}", print_map(&map));

        if iterations.get(&map).is_some() {
            println!("hi");
            if cycle_start {
                println!("{:?}", cycle.len());
                cycle_end = true;
                break;
            }
            cycle_start = true;
        }
        if cycle_start && !cycle_end {
            cycle.push(map.clone());
        }
        iterations.insert(map.clone(), iterations.len());
    }
    calculate_weights(&map)
}

fn tilt_cycle(mut map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut cloned_map = map.clone();
    let directions = vec![
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];
    for i in 0..4 {
        cloned_map = tilt(&map, &directions[i]);
    }
    cloned_map
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_ranges(map: &Vec<Vec<char>>, index: usize, direction: &Direction) -> Vec<[usize; 2]> {
    match direction {
        Direction::North => get_ranges_vertical(map, index),
        Direction::South => get_ranges_vertical(map, index),
        Direction::West => get_ranges_horizontal(map, index),
        Direction::East => get_ranges_horizontal(map, index),
    }
}

fn get_ranges_vertical(map: &Vec<Vec<char>>, collum: usize) -> Vec<[usize; 2]> {
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

fn get_ranges_horizontal(map: &Vec<Vec<char>>, row: usize) -> Vec<[usize; 2]> {
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

fn get_boulder_between_range(
    map: &Vec<Vec<char>>,
    index: usize,
    range: [usize; 2],
    direction: &Direction,
) -> Vec<usize> {
    let mut result = vec![];
    match direction {
        Direction::South | Direction::North => {
            for y in range[1]..=range[0] {
                if map[y][index] == 'O' {
                    result.push(y);
                }
            }
        }
        Direction::West | Direction::East => {
            for x in range[1]..=range[0] {
                if map[index][x] == 'O' {
                    result.push(x);
                }
            }
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

fn tilt(map: &Vec<Vec<char>>, direction: &Direction) -> Vec<Vec<char>> {
    match direction {
        Direction::North | Direction::South => tilt_vertically(map, direction),
        Direction::East | Direction::West => tilt_horizontally(map, direction),
    }
}

fn tilt_vertically(map: &Vec<Vec<char>>, direction: &Direction) -> Vec<Vec<char>> {
    let mut map_clone = generate_empty_map(map);
    for collum in 0..map[0].len() {
        let ranges = get_ranges(map, collum, direction);
        for range in ranges {
            let boulders = get_boulder_between_range(map, collum, range, direction);
            let (begining, increment) = match direction {
                Direction::North => (range[1], 1),
                Direction::South => (range[0], -1),
                _ => panic!("can't tilt vertically in horizontal direction"),
            };
            let starting: i32 = if (begining == 0 || begining == map.len() - 1)
                && map_clone[begining][collum] == '.'
            {
                begining as i32
            } else {
                begining as i32 + increment
            };
            for (i, _boulder) in boulders.iter().enumerate() {
                map_clone[(starting + i as i32 * increment) as usize][collum] = 'O';
            }
        }
    }
    map_clone
}

fn tilt_horizontally(map: &Vec<Vec<char>>, direction: &Direction) -> Vec<Vec<char>> {
    let mut map_clone = generate_empty_map(map);
    for row in 0..map.len() {
        let ranges = get_ranges(map, row, direction);
        for range in ranges {
            let boulders = get_boulder_between_range(map, row, range, direction);
            let (begining, increment) = match direction {
                Direction::West => (range[1], 1),
                Direction::East => (range[0], -1),
                _ => panic!("can't tilt vertically in horizontal direction"),
            };
            let starting: i32 = if (begining == 0 || begining == map[0].len() - 1)
                && map_clone[row][begining] == '.'
            {
                begining as i32
            } else {
                begining as i32 + increment
            };
            for (i, _boulder) in boulders.iter().enumerate() {
                map_clone[row][(starting + i as i32 * increment) as usize] = 'O';
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

fn print_map(map: &Vec<Vec<char>>) -> String {
    map.iter()
        .map(|row| row.iter().collect::<String>() + "\n")
        .collect()
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
        let result = get_ranges(&map, 0, &Direction::North);
        assert_eq!(
            result,
            vec![[map.len() - 1, map.len() - 2], [map.len() - 2, 0]]
        );
    }
    #[test]
    fn boulder_indexes_test() {
        let map = parse(TEST_INPUT);
        let ranges = get_ranges(&map, 0, &Direction::North);
        let boulders = get_boulder_between_range(&map, 0, ranges[1], &Direction::North);

        assert_eq!(boulders.len(), 4);
    }
    #[test]
    fn boulder_indexes_test_south() {
        let map = parse(TEST_INPUT);
        let ranges = get_ranges(&map, 0, &Direction::South);
        let boulders = get_boulder_between_range(&map, 0, ranges[1], &Direction::North);

        assert_eq!(boulders.len(), 4);
    }
    #[test]
    fn boulder_indexes_test_east() {
        let map = parse(TEST_INPUT);
        let ranges = get_ranges(&map, 3, &Direction::East);
        let boulders = get_boulder_between_range(&map, 1, ranges[1], &Direction::East);

        assert_eq!(boulders.len(), 0);
    }
    #[test]
    fn boulder_indexes_test_west() {
        let map = parse(TEST_INPUT);
        let ranges = get_ranges(&map, 3, &Direction::West);
        let boulders = get_boulder_between_range(&map, 1, ranges[1], &Direction::West);

        assert_eq!(boulders.len(), 0);
    }
    fn boulder_indexes_test_west_2() {
        let map = parse(TEST_INPUT);
        let ranges = get_ranges(&map, 0, &Direction::West);
        let boulders = get_boulder_between_range(&map, 1, ranges[1], &Direction::West);

        assert_eq!(boulders.len(), 3);
    }
    #[test]
    fn north_tilt_test() {
        let map = parse(TEST_INPUT);
        let expected = parse(TEST_RESULT);
        let result = tilt(&map, &Direction::North);
        assert_eq!(result, expected);
    }
    #[test]
    fn south_tilt_test() {
        let map = parse(TEST_INPUT);
        let expected_map = ".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O";
        let expected = parse(expected_map);
        let result = tilt(&map, &Direction::South);
        print_map(&result);
        assert_eq!(result, expected);
    }
    #[test]
    fn west_tilt_test() {
        let map = parse(TEST_INPUT);
        let expected_map = "O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#....";
        let expected = parse(expected_map);
        let result = tilt(&map, &Direction::West);
        print_map(&result);
        assert_eq!(result, expected);
    }
    #[test]
    fn east_tilt_test() {
        let map = parse(TEST_INPUT);
        let expected_map = "....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#....
";
        let expected = parse(expected_map);
        let result = tilt(&map, &Direction::East);
        print_map(&result);
        assert_eq!(result, expected);
    }
    #[test]
    fn one_cycle_test() {
        let map = parse(TEST_INPUT);
        let expected_map = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        let expected = parse(expected_map);
        let result = tilt_cycle(map);
        print_map(&result);
        assert_eq!(result, expected);
    }
    //         let map = parse(input);
    //         let result = tilt_north(&map);
    //         println!("map is {:?}", result);
    //         assert_eq!(result[2][0], 'O');
    //         assert_eq!(result[1][0], 'O');
    //     }
    //     #[test]
    //     fn part_1_test() {
    //         let result = solve(TEST_INPUT);
    //         assert_eq!(result, 136);
    //     }
    //     #[test]
    //     fn part_1_test_plus_1() {
    //         let result = solve(TEST_RESULT_PLUS_1);
    //         assert_eq!(result, 137);
    //     }
}
