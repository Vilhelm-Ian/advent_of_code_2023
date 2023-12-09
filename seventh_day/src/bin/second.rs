use std::cmp::Ordering;
use std::collections::HashMap;

mod input;

fn main() {
    let result = solve(input::INPUT);
    println!("{:?}", result);
}

fn solve(input: &str) -> usize {
    let mut ranks = parse_input(input);
    ranks.iter_mut().for_each(|rank| {
        rank.sort_by(|a, b| compare_hands(a.0, b.0));
    });
    ranks
        .iter()
        .flat_map(|rank| rank.iter().map(|(_hand, bid)| bid))
        .enumerate()
        .fold(0, |acc, (index, bid)| acc + (*bid as usize * (index + 1)))
}

fn parse_input(input: &str) -> Vec<Vec<([char; 5], i32)>> {
    let mut ranks = vec![vec![]; 8];
    input.lines().for_each(|line| {
        let mut splited = line.split(' ');
        let hand = build_hand(splited.next().unwrap());
        let bid = splited.next().unwrap().parse::<i32>().unwrap();
        let rank = get_hand_type(hand);
        ranks[rank as usize].push((hand, bid));
    });
    ranks
}

fn get_hand_type(hand: [char; 5]) -> i32 {
    let hand_name_later = how_many_are_same(hand);
    match hand_name_later {
        [0, 0, 0, 0, 5] => 7,
        [0, 0, 0, 1, 4] => 6,
        [0, 0, 0, 2, 3] => 5,
        [0, 0, 1, 1, 3] => 4,
        [0, 0, 1, 2, 2] => 3,
        [0, 1, 1, 1, 2] => 2,
        [1, 1, 1, 1, 1] => 1,
        _ => panic!("invalid hand "),
    }
}

fn how_many_are_same(hand: [char; 5]) -> [usize; 5] {
    let mut result = [0, 0, 0, 0, 0];
    let mut map = HashMap::new();
    let mut j = 0;
    hand.iter().for_each(|card| {
        if *card == 'J' {
            j += 1;
            return;
        }
        let card_entry = map.entry(card).or_insert(0);
        *card_entry += 1;
    });
    for (i, value) in map.values().enumerate() {
        result[i] = *value
    }
    let index_of_biggest = find_index_of_biggest(result);
    result[index_of_biggest] += j;
    result.sort();
    result
}

//I should change the name of the argument
fn find_index_of_biggest(hand: [usize; 5]) -> usize {
    hand.into_iter()
        .enumerate()
        .fold([0, 0], |[biggest_index, biggest], (card_index, card)| {
            if card > biggest {
                [card_index, card]
            } else {
                [biggest_index, biggest]
            }
        })[0]
}

fn build_hand(hand: &str) -> [char; 5] {
    let mut result = [' '; 5];
    hand.chars()
        .enumerate()
        .for_each(|(index, card)| result[index] = card);
    result
}

fn get_card_strength(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => card.to_digit(10).expect("couldn't get card stregth"),
    }
}

fn compare_hands(a: [char; 5], b: [char; 5]) -> Ordering {
    for i in 0..5 {
        let a_value = get_card_strength(a[i]);
        let b_value = get_card_strength(b[i]);
        if a_value > b_value {
            return Ordering::Greater;
        };
        if a_value < b_value {
            return Ordering::Less;
        };
    }
    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = solve(input);
        assert_eq!(5905, result);
    }
}
