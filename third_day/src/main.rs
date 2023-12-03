mod input;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("{:?}", solve_part_1(input::INPUT));
    println!("{:?}", solve_part_2(input::INPUT));
}

fn solve_part_1(input: &str) -> i32 {
    let mut numbers = vec![];
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for y in 0..lines.len() {
        let mut number = String::from("");
        let mut is_part_number = false;
        for x in 0..lines[y].len() {
            if lines[y][x].is_ascii_digit() {
                number.push(lines[y][x]);
                if !is_part_number && is_part_digit(&lines, y, x) {
                    is_part_number = true;
                };
            }
            if !lines[y][x].is_ascii_digit() || x == lines[y].len() - 1 {
                if let Ok(parsed_number) = number.parse::<i32>() {
                    if is_part_number {
                        numbers.push(parsed_number);
                    }
                }
                is_part_number = false;
                number = String::from("");
            }
        }
    }
    numbers.iter().sum()
}

fn is_part_digit(map: &Vec<Vec<char>>, row: usize, collum: usize) -> bool {
    for row in iterate_around_index(map, row, collum) {
        for letter in row {
            match letter {
                ' ' | '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => (),
                _ => {
                    return true;
                }
            }
        }
    }
    false
}

fn iterate_around_index(map: &Vec<Vec<char>>, row: usize, collum: usize) -> [[char; 3]; 3] {
    let mut chars = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];
    for y in 0..3 {
        let y = y - 1;
        if (y + row as i32) == -1 || (y + row as i32) == map.len() as i32 {
            continue;
        }
        for x in 0..3 {
            let x = x - 1;
            if (x + collum as i32) == -1 || (x + collum as i32) == map[0].len() as i32 {
                continue;
            }
            if y == 0 && x == 0 {
                continue;
            }
            let row = (row as i32 + y) as usize;
            let collum = (collum as i32 + x) as usize;
            let y = (y + 1) as usize;
            let x = (x + 1) as usize;
            chars[y][x] = map[row][collum];
        }
    }
    chars
}

fn solve_part_2(input: &str) -> usize {
    // the key is the star index and the value it's an array where the first value is
    // how many numbers it touches, and the secondi is the ratio
    let mut stars: HashMap<[usize; 2], [usize; 2]> = HashMap::new();
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for y in 0..lines.len() {
        let mut number = String::from("");
        for x in 0..lines[y].len() {
            if lines[y][x].is_ascii_digit() {
                number.push(lines[y][x]);
            }
            if !lines[y][x].is_ascii_digit() || x == lines[y].len() - 1 {
                if let Ok(parsed_number) = number.parse::<i32>() {
                    let stars_around_number = find_stars_around_number(&lines, y, x, &number);
                    for star_index in stars_around_number {
                        update_star(&mut stars, star_index, parsed_number);
                    }
                    number = String::from("");
                }
            }
        }
    }
    sum_cogs(&stars)
}

fn find_stars_around_number(
    map: &Vec<Vec<char>>,
    row: usize,
    collum: usize,
    number: &String,
) -> HashSet<[usize; 2]> {
    let mut stars_indexs: HashSet<[usize; 2]> = HashSet::new();
    for i in (collum - number.len())..collum {
        let letters_around = iterate_around_index(map, row, i);
        for y in 0..letters_around.len() {
            for x in 0..letters_around[0].len() {
                if letters_around[y][x] == '*' {
                    stars_indexs.insert([row + y - 1, i + x - 1]);
                }
            }
        }
    }
    stars_indexs
}

fn sum_cogs(stars: &HashMap<[usize; 2], [usize; 2]>) -> usize {
    let mut result = 0;
    for value in stars.values() {
        if value[0] == 2 {
            result += value[1]
        }
    }
    result
}

fn update_star(stars: &mut HashMap<[usize; 2], [usize; 2]>, star_index: [usize; 2], number: i32) {
    let star = stars.entry(star_index).or_insert([0, 1]);
    if star[0] < 3 {
        (*star)[0] += 1;
        (*star)[1] *= number as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_test_input() {
        let result = solve_part_1(input::TEST_INPUT_PART_1);
        assert_eq!(result, 4361);
    }
    #[test]
    fn part_1_from_input() {
        let input = "..756....
.*.......
700..507*";
        let result = solve_part_1(input);
        assert_eq!(result, 1963);
    }
    #[test]
    fn second_part_1_from_input() {
        let input = "....
....
.485
...*";
        let result = solve_part_1(input);
        assert_eq!(result, 485);
    }
    #[test]
    fn part_2_test_input() {
        let result = solve_part_2(input::TEST_INPUT_PART_2);
        assert_eq!(result, 467835);
    }
}
