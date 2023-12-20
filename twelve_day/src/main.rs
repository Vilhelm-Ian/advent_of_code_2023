use rayon::prelude::*;
use std::collections::HashSet;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod input;

fn main() {
    let input = "???.### 1,1,3";
    solve_part_2(input);

    // let result = part_2_input_generate(input::INPUT);
    // println!("{:?}", result);
    // let now = Instant::now();
    // let result = how_many_arragments(input::INPUT);
    // println!("{:?}", result);
    // println!("{} seconds", now.elapsed().as_secs());
}

fn solve_part_2(input: &str) -> usize {
    let input = part_2_input_generate(input);
    how_many_arragments(input.as_str())
}

fn part_2_input_generate(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let splitted: Vec<&str> = line.split(' ').collect();
            let first_part = (0..5).fold(String::new(), |acc, i| {
                if i != 4 {
                    acc + splitted[0] + "?"
                } else {
                    acc + splitted[0]
                }
            });
            let second_part = (0..5).fold(String::new(), |acc, i| {
                if i != 4 {
                    acc + splitted[1] + ","
                } else {
                    acc + splitted[1]
                }
            });
            let mut result = first_part + " " + second_part.as_str();
            result.push('\n');
            result
        })
        .collect()
}

fn how_many_arragments(input: &str) -> usize {
    let rows = parse_input(input);
    // rows.iter()
    //     .enumerate()
    //     .map(|(i, row)| {
    //         let values: Vec<usize> = row[1]
    //             .split(',')
    //             .map(|x| x.parse::<usize>().unwrap())
    //             .collect();
    //         let mut set = HashSet::new();
    //         let mut seen = HashSet::new();
    //         let number_of_arragements =
    //             insert_all(row[0], &values.clone(), values, &mut set, &mut seen);
    //         number_of_arragements
    //     })
    //     .sum()
    let result = rows
        .par_iter()
        .fold(
            || 0,
            |acc, row| {
                let values: Vec<usize> = row[1]
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                let mut set = HashSet::new();
                let mut seen = HashSet::new();
                let number_of_arragements =
                    insert_all(row[0], &values.clone(), values, &mut set, &mut seen);

                acc + number_of_arragements
            },
        )
        .reduce(|| 0, |current, acc| current + acc);
    result
}

fn insert_all(
    springs: &str,
    original_records: &Vec<usize>,
    records: Vec<usize>,
    set: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) -> usize {
    if seen.get(&springs.to_string()).is_some() {
        return 0;
    }
    seen.insert(springs.to_string());
    let unknows = all_unknwons(springs);
    let result = springs.replace('?', ".");
    if &how_many_broken_sequences(result.as_str()) == original_records {
        set.insert(result);
    }
    println!("{:?}", springs);
    for record in records.iter() {
        for unknown in unknows.iter() {
            if let Some(updated_spring) = insert_broken(springs, *unknown, *record) {
                let records = (records[1..records.len()]).to_vec();
                insert_all(
                    updated_spring.as_str(),
                    original_records,
                    records,
                    set,
                    seen,
                );
            };
        }
    }

    if set.is_empty() {
        1
    } else {
        set.len()
    }
}

fn all_unknwons(springs: &str) -> Vec<usize> {
    let mut result = vec![];
    for i in 0..springs.len() {
        if &springs[i..i + 1] == "?" {
            result.push(i);
        };
    }
    result
}

fn how_many_broken_sequences(springs: &str) -> Vec<usize> {
    let mut result = vec![];
    let spltted_spring: Vec<char> = springs.chars().collect();
    let mut current_sequence_length = 0;
    for (i, _) in spltted_spring.iter().enumerate() {
        if spltted_spring[i] == '#' {
            current_sequence_length += 1;
            if i + 1 == springs.len() || spltted_spring[i + 1] != '#' {
                result.push(current_sequence_length);
                current_sequence_length = 0;
            }
        }
    }
    result
}

fn insert_broken(springs: &str, mut index: usize, mut amount: usize) -> Option<String> {
    let mut springs_char = springs.chars().collect::<Vec<char>>();
    let mut starting: i32 = 0;
    while index > starting as usize && springs_char[index - starting as usize - 1] == '#' {
        starting += 1;
    }
    let adjusted_index = (index as i32 - starting) as usize;

    for i in 0..amount {
        if adjusted_index + i == springs_char.len() {
            return None;
        }
        if springs_char[adjusted_index + i] == '.' {
            return None;
        }
        if adjusted_index != 0 && springs_char[adjusted_index - 1] == '?' {
            springs_char[adjusted_index - 1] = '.';
        }
        springs_char[adjusted_index + i] = '#';
    }
    if adjusted_index + amount < springs_char.len() {
        if springs_char[adjusted_index + amount] == '#' {
            return None;
        }
        springs_char[adjusted_index + amount] = '.';
    }
    Some(springs_char.iter().collect::<String>())
}

fn parse_input(input: &str) -> Vec<[&str; 2]> {
    input
        .lines()
        .map(|x| {
            let mut splited = x.split(' ');
            [splited.next().unwrap(), splited.next().unwrap()]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_line_part_1() {
        let input = "???.### 1,1,3";
        let result = how_many_arragments(input);
        assert_eq!(1, result);
    }
    #[test]
    fn test_second_line_part_1() {
        let input = ".??..??...###. 1,1,3";
        let result = how_many_arragments(input);
        assert_eq!(4, result);
    }
    #[test]
    fn test_third_line_part_1() {
        let input = "?###???????? 3,2,1";
        let result = how_many_arragments(input);
        assert_eq!(10, result);
    }
    #[test]
    fn test_fort_line_part_1() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        let result = how_many_arragments(input);
        assert_eq!(1, result);
    }
    #[test]
    fn test_fifth_line_part_1() {
        let input = "????.#...#... 4,1,1";
        let result = how_many_arragments(input);
        assert_eq!(1, result);
    }
    #[test]
    fn part_1_test() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let result = how_many_arragments(input);
        assert_eq!(21, result);
    }
    #[test]
    fn part_2_test() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let result = solve_part_2(input);
        assert_eq!(525152, result);
    }
    #[test]
    fn input_test_1() {
        let input = "???#...#.?#??#?#? 1,1,1,8";
        let result = how_many_arragments(input);
        assert_eq!(2, result);
    }
    #[test]
    fn input_test_2() {
        let input = "#?.???????#????#???. 1,1,12";
        let result = how_many_arragments(input);
        assert_eq!(6, result);
    }
    #[test]
    fn input_test_3() {
        let input = "????.#??##??? 1,2,3";
        let result = how_many_arragments(input);
        assert_eq!(4, result);
    }
    #[test]
    fn input_test_4() {
        let input = "??????.?.???##?#?. 6,1,7";
        let result = how_many_arragments(input);
        assert_eq!(2, result);
    }
    #[test]
    fn input_test_5() {
        let input = "#??????????##?.??? 5,1,1,3,1,1";
        let result = how_many_arragments(input);
        assert_eq!(4, result);
    }
    #[test]
    fn input_test_6() {
        let input = "??.#?.?.?#?.? 1,1";
        let result = how_many_arragments(input);
        assert_eq!(1, result);
    }
    #[test]
    fn broken_sequence_test_1() {
        let input = "#..#.############...";
        let result = how_many_broken_sequences(input);
        assert_eq!(vec![1, 1, 12], result);
    }
    #[test]
    fn insert_test_1() {
        let input = "#.??.#??##???";
        let result = insert_broken(input, 6, 2);
        assert_eq!(Some("#.??.##.##???".to_string()), result);
    }
    #[test]
    fn parse_input_2() {
        let input = ".# 1";
        let result = part_2_input_generate(input);
        assert_eq!(result, ".#?.#?.#?.#?.# 1,1,1,1,1\n");
    }
}
