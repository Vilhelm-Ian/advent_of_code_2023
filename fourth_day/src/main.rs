mod input;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input.split(':').collect::<Vec<&str>>()[1]
        .split('|')
        .map(|numbers| numbers.split(' ').filter(|x| x != &"").collect())
        .collect()
}

fn get_card_points(input: &str) -> i32 {
    let card_lists = parse_input(input);
    let mut winning_numbers = HashSet::new();
    card_lists[0].iter().for_each(|number| {
        winning_numbers.insert(number);
    });
    let winning_numbers_in_second_list: Vec<&&str> = card_lists[1]
        .iter()
        .filter(|number| winning_numbers.get(number).is_some())
        .collect();
    if winning_numbers_in_second_list.len() == 0 {
        return 0;
    }
    2_i32.pow(winning_numbers_in_second_list.len() as u32 - 1)
}

fn solve_part_1(input: &str) -> i32 {
    input.lines().map(get_card_points).sum()
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
}
