mod input;

use std::collections::HashSet;

fn main() {
    let result = solve(input::INPUT);
    println!("{:?}", result);
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .collect()
}

fn move_right(map: &mut [Vec<char>], distance: usize, location: [usize; 2]) {
    let number_of_new_collums = location[1] as i32 + distance as i32 + 1 - map[0].len() as i32;
    if number_of_new_collums > 0 {
        let new_collums = vec!['.'; number_of_new_collums as usize];
        map.iter_mut()
            .for_each(|row| row.extend(new_collums.iter()));
    }
    map[location[0]]
        .iter_mut()
        .skip(location[1])
        .take(distance + 1)
        .for_each(|square| *square = '#');
}

fn move_left(map: &mut [Vec<char>], distance: usize, location: [usize; 2]) {
    map.iter_mut().for_each(|row| row.reverse());
    let mirrored_location = [location[0], map[0].len() - 1 - location[1]];
    move_right(map, distance, mirrored_location);
    map.iter_mut().for_each(|row| row.reverse());
}

fn move_down(map: &mut Vec<Vec<char>>, distance: usize, location: [usize; 2]) {
    let number_of_new_rows = location[0] as i32 + distance as i32 + 1 - map.len() as i32;
    if number_of_new_rows > 0 {
        let new_rows = vec![vec!['.'; map[0].len()]; number_of_new_rows as usize];
        map.extend(new_rows);
    }
    map.iter_mut()
        .skip(location[0])
        .take(distance + 1)
        .for_each(|row| row[location[1]] = '#');
}

fn move_up(map: &mut Vec<Vec<char>>, distance: usize, location: [usize; 2]) {
    map.reverse();
    let mirrored_location = [map.len() - 1 - location[0], location[1]];
    move_down(map, distance, mirrored_location);
    map.reverse();
}

fn update_location(
    location: &mut [usize; 2],
    offset: [i32; 2],
    other_location: [usize; 2],
    map_changed: bool,
) {
    if map_changed {
        *location = other_location;
    } else {
        *location = [
            (location[0] as i32 + offset[0]) as usize,
            (location[1] as i32 + offset[1]) as usize,
        ];
    }
}

fn draw_outline(instructions: Vec<Vec<&str>>) -> Vec<Vec<char>> {
    let mut result = vec![vec!['#']];
    let mut location = [0, 0];
    for instruciton in instructions {
        let distance = instruciton[1].parse::<usize>().expect("invalid distance");
        // print_grid(&result);
        match instruciton[0] {
            "D" => {
                let old_length = result.len();
                move_down(&mut result, distance, location);
                let new_length = result.len();
                let did_map_change = old_length != new_length;
                let current_collum = location[1];
                update_location(
                    &mut location,
                    [distance as i32, 0],
                    [result.len() - 1, current_collum],
                    did_map_change,
                )
            }
            "U" => {
                let old_length = result.len();
                move_up(&mut result, distance, location);
                let new_length = result.len();
                let did_map_change = old_length != new_length;
                let current_collum = location[1];
                update_location(
                    &mut location,
                    [-(distance as i32), 0],
                    [0, current_collum],
                    did_map_change,
                )
            }
            "L" => {
                let old_length = result[0].len();
                move_left(&mut result, distance, location);
                let new_length = result[0].len();
                let did_map_change = old_length != new_length;
                let current_row = location[0];
                update_location(
                    &mut location,
                    [0, -(distance as i32)],
                    [current_row, 0],
                    did_map_change,
                )
            }
            "R" => {
                let old_length = result[0].len();
                move_right(&mut result, distance, location);
                let new_length = result[0].len();
                let did_map_change = old_length != new_length;
                let current_row = location[0];
                update_location(
                    &mut location,
                    [0, distance as i32],
                    [current_row, result[0].len() - 1],
                    did_map_change,
                )
            }
            _ => panic!("invalid direction"),
        }
    }
    result
}

fn tranverse_grid(
    visited_places: &mut HashSet<[usize; 2]>,
    starting_location: [usize; 2],
    grid: &Vec<Vec<char>>,
) {
    if grid[starting_location[0]][starting_location[1]] == '#'
        || visited_places.get(&starting_location).is_some()
    {
        return;
    }
    visited_places.insert(starting_location);
    if starting_location[0] > 0 {
        let new_location = [starting_location[0] - 1, starting_location[1]];
        tranverse_grid(visited_places, new_location, grid);
    }
    if starting_location[0] < grid.len() - 1 {
        let new_location = [starting_location[0] + 1, starting_location[1]];
        tranverse_grid(visited_places, new_location, grid);
    }
    if starting_location[1] > 0 {
        let new_location = [starting_location[0], starting_location[1] - 1];
        tranverse_grid(visited_places, new_location, grid);
    }
    if starting_location[1] < grid[0].len() - 1 {
        let new_location = [starting_location[0], starting_location[1] + 1];
        tranverse_grid(visited_places, new_location, grid);
    }
}

fn calculate_area_outside_trenches(grid: &Vec<Vec<char>>) -> usize {
    let mut visited_places = HashSet::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, _square) in row.iter().enumerate() {
            if y == 0 || y == grid.len() - 1 || x == 0 || x == grid[0].len() - 1 {
                tranverse_grid(&mut visited_places, [y, x], grid);
            }
        }
    }
    visited_places.len()
}

fn solve(input: &str) -> usize {
    let instructions = parse(input);
    let field = draw_outline(instructions);
    let area_outside_trenches = calculate_area_outside_trenches(&field);
    let field_area = field.len() * field[0].len();
    print_grid(&field);
    field_area - area_outside_trenches
}

fn print_grid(grid: &Vec<Vec<char>>) {
    let result = grid
        .iter()
        .map(|row| row.iter().collect::<String>() + "\n")
        .collect::<String>();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    pub const OUTLINE: &str = "#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######";
    #[test]
    fn move_right_test() {
        let mut result = vec![vec!['#', '#'], vec!['.', '.']];
        move_right(&mut result, 3, [0, 1]);
        let expected = vec![vec!['#', '#', '#', '#', '#'], vec!['.', '.', '.', '.', '.']];
        assert_eq!(result, expected);
    }
    #[test]
    fn move_right_test_2() {
        let mut result = vec![vec!['.', '#'], vec!['.', '.']];
        move_right(&mut result, 3, [0, 1]);
        let expected = vec![vec!['.', '#', '#', '#', '#'], vec!['.', '.', '.', '.', '.']];
        assert_eq!(result, expected);
    }
    #[test]
    fn move_left_test() {
        let mut result = vec![vec!['#', '.'], vec!['.', '.']];
        move_left(&mut result, 3, [0, 0]);
        let expected = vec![vec!['#', '#', '#', '#', '.'], vec!['.', '.', '.', '.', '.']];
        assert_eq!(result, expected);
    }
    #[test]
    fn move_down_test() {
        let mut result = vec![vec!['#', '#'], vec!['.', '.']];
        move_down(&mut result, 3, [0, 1]);
        let expected = vec![
            vec!['#', '#'],
            vec!['.', '#'],
            vec!['.', '#'],
            vec!['.', '#'],
        ];
        assert_eq!(result, expected);
    }
    #[test]
    fn move_up_test() {
        let mut result = vec![vec!['#', '#'], vec!['.', '.']];
        move_up(&mut result, 3, [0, 1]);
        let expected = vec![
            vec!['.', '#'],
            vec!['.', '#'],
            vec!['.', '#'],
            vec!['#', '#'],
            vec!['.', '.'],
        ];
        assert_eq!(result, expected);
    }
    #[test]
    fn outline_test() {
        let instructions = parse(TEST_INPUT);
        let result = draw_outline(instructions);
        let expected: Vec<Vec<char>> = OUTLINE.lines().map(|row| row.chars().collect()).collect();
        assert_eq!(result, expected);
    }
    #[test]
    fn part_1_test() {
        let result = solve(TEST_INPUT);
        assert_eq!(result, 62);
    }
}
