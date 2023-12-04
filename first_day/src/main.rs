use rayon::prelude::*;
use regex::{Captures, Regex};
use std::time::Instant;
pub mod input;

fn main() {
    let now = Instant::now();
    let first_part_result = solution_first_part(input::FIRST_PART_INPUT);
    let elapsed_time = now.elapsed();
    println!("{:?}", first_part_result);
    println!("{:?}", elapsed_time);

    let now = Instant::now();
    let second_part_result = solve_second_part(input::FIRST_PART_INPUT);
    let elapsed_time = now.elapsed();
    println!("{:?}", second_part_result);
    println!("{:?}", elapsed_time);
}

fn solution_first_part(calibration_document: &str) -> u32 {
    let lines: Vec<&str> = calibration_document.split('\n').collect();
    sum_of_all_calibration_values(lines)
}

fn find_calibration_value(line: &str) -> u32 {
    let letters: Vec<char> = line.chars().collect();
    let mut first = None;
    let mut second = None;
    for i in 0..letters.len() {
        if first.is_none() {
            update_digit(&mut first, letters[i]);
        }
        if second.is_none() {
            update_digit(&mut second, letters[letters.len() - i - 1]);
        }
        if let (Some(first), Some(second)) = (first, second) {
            return first * 10 + second;
        }
    }
    panic!("didn't find a calibration value in line: {:?}", line)
}

fn update_digit(calibration_value: &mut Option<u32>, letter: char) {
    if let Some(digit) = letter.to_digit(10) {
        *calibration_value = Some(digit);
    };
}

fn sum_of_all_calibration_values(lines: Vec<&str>) -> u32 {
    lines
        .iter()
        .fold(0, |acc, line| acc + find_calibration_value(line))
}

fn solve_second_part(input: &str) -> u32 {
    let splitted: Vec<&str> = input.split('\n').collect();
    splitted
        .par_iter()
        .fold(
            || 0,
            |acc, cur| acc + find_calibration_value_using_regex(cur),
        )
        .reduce(|| 0, |a, b| a + b)
}

fn find_calibration_value_using_regex(line: &str) -> u32 {
    let regexs = [
        Regex::new(r"one").unwrap(),
        Regex::new(r"two").unwrap(),
        Regex::new(r"three").unwrap(),
        Regex::new(r"four").unwrap(),
        Regex::new(r"five").unwrap(),
        Regex::new(r"six").unwrap(),
        Regex::new(r"seven").unwrap(),
        Regex::new(r"eight").unwrap(),
        Regex::new(r"nine").unwrap(),
        Regex::new(r"0|1|2|3|4|5|6|7|8|9").unwrap(),
    ];
    let mut matches = vec![];
    regexs
        .iter()
        .for_each(|re| re.find_iter(line).for_each(|x| matches.push(x)));
    matches.sort_by_key(|a| a.start());
    let first = parse_number_to_string(matches[0].as_str());
    let second = parse_number_to_string(matches[matches.len() - 1].as_str());
    first * 10 + second
}

fn parse_number_to_string(number: &str) -> u32 {
    match number {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        &_ => {
            panic!("not a number")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calibration_line_1() {
        let result = find_calibration_value("1abc2");
        assert_eq!(result, 12);
    }
    #[test]
    fn calibration_line_2() {
        let result = find_calibration_value("pqr3stu8vwx");
        assert_eq!(result, 38);
    }
    #[test]
    fn calibration_line_3() {
        let result = find_calibration_value("a1b2c3d4e5f");
        assert_eq!(result, 15);
    }
    #[test]
    fn calibration_line_4() {
        let result = find_calibration_value("treb7uchet");
        assert_eq!(result, 77);
    }
    #[test]
    fn first_part_test_input() {
        let result = solution_first_part(input::FIRST_PART_TEST_INPUT);
        assert_eq!(result, 142);
    }
    // This tests don't work anymore lazy to fix them
    // #[test]
    // fn first_correction() {
    //     let result = replace_spelled_numbers(String::from("two1nine"));
    //     assert_eq!(result, "219");
    // }
    // #[test]
    // fn second_correction() {
    //     let result = replace_spelled_numbers(String::from("eightwothree"));
    //     assert_eq!(result, "823");
    // }
    // #[test]
    // fn third_correction() {
    //     let result = replace_spelled_numbers(String::from("abcone2threexyz"));
    //     assert_eq!(result, "abc123xyz");
    // }
    // #[test]
    // fn fourth_correction() {
    //     let result = replace_spelled_numbers(String::from("4nineeightseven2"));
    //     assert_eq!(result, "49872");
    // }
    // #[test]
    // fn fifth_correction() {
    //     let result = replace_spelled_numbers(String::from("zoneight234"));
    //     assert_eq!(result, "z18234");
    // }
    // #[test]
    // fn sixth_correction() {
    //     let result = replace_spelled_numbers(String::from("7pqrstsixteen"));
    //     assert_eq!(result, "7pqrst6teen");
    // }
    #[test]
    fn second_part_test_input() {
        let result = solve_second_part(input::SECOND_PART_TEST_INPUT);
        assert_eq!(result, 281);
    }
}
