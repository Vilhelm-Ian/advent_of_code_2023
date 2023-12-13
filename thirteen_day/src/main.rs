//task where the mirrors are
// use rayon::prelude::*;

mod input;

fn main() {
    let result = solve(input::INPUT);
    println!("{result}");
    let result = solve_2(input::INPUT);
    println!("{result}");
}

fn solve_2(input: &str) -> i32 {
    let vallies: Vec<&str> = input.split("\n\n").collect();
    vallies
        .iter()
        .map(|valley| {
            let valley = valley.lines().map(|line| line.to_string()).collect();
            if let Some(number) = compare_row_2(&valley) {
                return number;
            };
            if let Some(number) = compare_collum_2(&valley) {
                return number;
            }
            panic!("couldn't find mirror");
        })
        .sum()
}

fn solve(input: &str) -> i32 {
    let vallies: Vec<&str> = input.split("\n\n").collect();
    vallies
        .iter()
        .map(|valley| {
            let valley = valley.lines().map(|line| line.to_string()).collect();
            if let Some(number) = compare_row(&valley) {
                return number;
            };
            if let Some(number) = compare_collum(&valley) {
                return number;
            }
            panic!("couldn't find mirror");
        })
        .sum()
}

fn compare_collum(valley: &Vec<String>) -> Option<i32> {
    let row = vec![' '; valley.len()];
    let mut new_valley = vec![row; valley[0].len()];
    valley.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, letter)| {
            new_valley[x][y] = letter;
        })
    });
    let flipped = new_valley
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect();

    compare_row(&flipped).map(|x| x / 100)
}

fn compare_row(valley: &Vec<String>) -> Option<i32> {
    for i in 1..valley.len() / 2 + 1 {
        let a = valley[0..i].to_vec();
        let mut b = valley[i..i + i].to_vec();
        b.reverse();
        if a == b {
            return Some(i as i32 * 100);
        }
        let mut reversed_valley = valley.clone();
        reversed_valley.reverse();
        let a = reversed_valley[0..i].to_vec();
        let mut b = reversed_valley[i..i + i].to_vec();
        b.reverse();
        if a == b {
            for (i, row) in reversed_valley.iter().enumerate() {
                let mut arrow = "";
                if i == 3 {
                    arrow = "<";
                }
            }
            return Some((reversed_valley.len() - i) as i32 * 100);
        }
    }
    None
}

fn compare_collum_2(valley: &Vec<String>) -> Option<i32> {
    let row = vec![' '; valley.len()];
    let mut new_valley = vec![row; valley[0].len()];
    valley.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, letter)| {
            new_valley[x][y] = letter;
        })
    });
    let flipped = new_valley
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect();
    compare_row_2(&flipped).map(|x| x / 100)
}

fn compare_row_2(valley: &Vec<String>) -> Option<i32> {
    for i in 1..valley.len() / 2 + 1 {
        let a = valley[0..i].to_vec();
        let mut b = valley[i..i + i].to_vec();
        b.reverse();
        let reflection_a = a
            .iter()
            .flat_map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        let reflection_b = b
            .iter()
            .flat_map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        if find_smudge(&reflection_a, &reflection_b).is_some() {
            return Some(i as i32 * 100);
        }
        let mut reversed_valley = valley.clone();
        reversed_valley.reverse();
        let a = reversed_valley[0..i].to_vec();
        let mut b = reversed_valley[i..i + i].to_vec();
        b.reverse();
        let reflection_a = a
            .iter()
            .flat_map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        let reflection_b = b
            .iter()
            .flat_map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        if find_smudge(&reflection_a, &reflection_b).is_some() {
            return Some((reversed_valley.len() - i) as i32 * 100);
        }
    }
    None
}

fn find_smudge(reflection_a: &Vec<char>, reflection_b: &Vec<char>) -> Option<[Vec<char>; 2]> {
    let mut counter = 0;
    for i in 0..reflection_a.len() {
        if reflection_a[i] != reflection_b[i] {
            counter += 1;
            if counter > 1 {
                break;
            }
        }
    }
    if counter == 1 {
        return Some([reflection_a.clone(), reflection_b.clone()]);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIRST_INPUT: &str = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    const SECOND_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    const FULL_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    const INPUT_1: &str = ".#..####..#
#....##....
..##....##.
#..#....#..
##..####..#
.#..####..#
#####..####";
    const INPUT_2: &str = ".#..####..#
#....##....
..##....##.
#..#....##.
##..####..#
.#..####..#
#####..####";
    #[test]
    fn horizontal_test() {
        let input = FIRST_INPUT.lines().map(|line| line.to_string()).collect();
        let result = compare_row(&input);
        assert_eq!(result.unwrap(), 400);
    }
    #[test]
    fn horizontal_test_2() {
        let input = SECOND_INPUT.lines().map(|line| line.to_string()).collect();
        let result = compare_row_2(&input);
        assert_eq!(result.unwrap(), 300);
    }
    #[test]
    fn horizontal_test_3() {
        let input = FIRST_INPUT.lines().map(|line| line.to_string()).collect();
        let result = compare_row_2(&input);
        assert_eq!(result.unwrap(), 100);
    }
    #[test]
    fn vertical_test() {
        let input = SECOND_INPUT.lines().map(|line| line.to_string()).collect();
        let result = compare_collum(&input);
        assert_eq!(result.unwrap(), 5);
    }
    #[test]
    fn part_2_test() {
        let result = solve_2(FULL_INPUT);
        assert_eq!(400, result);
    }
}
