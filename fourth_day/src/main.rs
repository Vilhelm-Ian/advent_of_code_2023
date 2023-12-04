mod input;
use core::num;
use std::collections::{HashMap, HashSet};

fn main() {
    let result_1 = solve_part_1(input::INPUT);
    println!("{result_1}");
    let result_2 = solve_part_2(input::INPUT);
    println!("{result_2}");
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input.split(':').collect::<Vec<&str>>()[1]
        .split('|')
        .map(|numbers| numbers.split(' ').filter(|x| x != &"").collect())
        .collect()
}

fn how_many_winning_numbers(card_lists: Vec<Vec<&str>>) -> usize {
    let mut winning_numbers = HashSet::new();
    card_lists[0].iter().for_each(|number| {
        winning_numbers.insert(number);
    });
    card_lists[1]
        .iter()
        .filter(|number| winning_numbers.get(number).is_some())
        .collect::<Vec<&&str>>()
        .len()
}

fn get_card_points(input: &str) -> i32 {
    let card_lists = parse_input(input);
    let number_of_winning_numbers = how_many_winning_numbers(card_lists) as u32;
    if number_of_winning_numbers == 0 {
        return 0;
    }
    2_i32.pow(number_of_winning_numbers - 1)
}

fn solve_part_1(input: &str) -> i32 {
    input.lines().map(get_card_points).sum()
}

fn solve_part_2(input: &str) -> i32 {
    let cards: Vec<&str> = input.lines().collect();
    let mut result = vec![1; cards.len()];
    cards
        .into_iter()
        .map(parse_input)
        .map(how_many_winning_numbers)
        .enumerate()
        .for_each(|(index, points)| {
            (index + 1..=index + points).for_each(|index_2| result[index_2] += result[index])
        });
    // card_lists.iter().map(|card| how_many_winning_numbers(card));
    result.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_line_test_input() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = get_card_points(input);
        assert_eq!(result, 8);
    }
    #[test]
    fn test_input() {
        let result = solve_part_1(input::TEST_INPUT);
        assert_eq!(result, 13);
    }
    #[test]
    fn test_input_part_2() {
        let result = solve_part_2(input::TEST_INPUT);
        assert_eq!(result, 30);
    }
}
