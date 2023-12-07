use std::cmp::Ordering;
use std::collections::HashMap;

mod input;

fn main() {
    let result = solve(input::INPUT);
    println!("{:?}", result);
}

fn solve(input: &str) -> usize {
    let mut ranks = vec![vec![]; 8];
    for line in input.lines() {
        let mut splited = line.split(' ');
        let hand = build_hand(splited.next().unwrap());
        let bid = splited.next().unwrap().parse::<i32>().unwrap();
        let rank = get_hand_type(hand);
        ranks[rank as usize].push((hand, bid));
    }
    ranks.iter_mut().for_each(|rank| {
        rank.sort_by(|a, b| {
            if a.0 == b.0 {
                return Ordering::Equal;
            }
            if is_card_bigger(a.0, b.0) {
                // it needs to be sorted in descending order
                return Ordering::Greater;
            }
            Ordering::Less
        })
    });
    let mut bids = vec![];
    let mut hands = vec![];
    ranks
        .iter()
        .for_each(|rank| rank.iter().for_each(|(_hand, bid)| bids.push(bid)));
    ranks
        .iter()
        .for_each(|rank| rank.iter().for_each(|(hand, _bid)| hands.push(hand)));
    bids.iter()
        .enumerate()
        .fold(0, |acc, (index, bid)| acc + (**bid as usize * (index + 1)))
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
    let mut biggest = 0;
    let mut index = 0;
    for i in 0..5 {
        if hand[i] > biggest {
            biggest = hand[i];
            index = i;
        }
    }
    index
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

fn is_card_bigger(a: [char; 5], b: [char; 5]) -> bool {
    for i in 0..5 {
        let a_value = get_card_strength(a[i]);
        let b_value = get_card_strength(b[i]);
        if a_value > b_value {
            return true;
        }
        if a_value < b_value {
            return false;
        }
    }
    false
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
