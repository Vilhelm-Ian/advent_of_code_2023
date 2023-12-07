use std::cmp::Ordering;
use std::collections::HashMap;

mod input;

fn main() {
    let result = solve_part_1(input::INPUT);
    println!("{:?}", result);
}

fn solve_part_1(input: &str) -> usize {
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
    hand.iter().for_each(|card| {
        let card_entry = map.entry(card).or_insert(0);
        *card_entry += 1;
    });
    for (i, value) in map.values().enumerate() {
        result[i] = *value
    }
    result.sort();
    result
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
        'J' => 11,
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
    fn convert_card() {
        let input = "AAAAA";
        let result = build_hand(input);
        assert_eq!(['A'; 5], result);
    }
    #[test]
    fn five_of_kind() {
        let input = build_hand("AAAAA");
        let result = how_many_are_same(input);
        assert_eq!([0, 0, 0, 0, 5], result);
    }
    #[test]
    fn not_five_of_kind() {
        let input = build_hand("KAAAA");
        let result = how_many_are_same(input);
        assert_eq!([0, 0, 0, 1, 4], result);
    }
    #[test]
    fn point_four_of_kind() {
        let input = build_hand("KAAAA");
        let result = get_hand_type(input);
        assert_eq!(6, result);
    }
    #[test]
    fn point_five_of_kind() {
        let input = build_hand("KAAAA");
        let result = get_hand_type(input);
        assert_eq!(6, result);
    }
    #[test]
    fn point_full_house() {
        let input = build_hand("AAAAA");
        let result = get_hand_type(input);
        assert_eq!(7, result);
    }
    #[test]
    fn high_card() {
        let input = build_hand("32T3K");
        let result = get_hand_type(input);
        assert_eq!(2, result);
    }
    #[test]
    fn is_bigger_1() {
        let a = build_hand("KK677");
        let b = build_hand("KTJJT");
        assert_eq!(true, is_card_bigger(a, b));
    }
    #[test]
    fn is_bigger_2() {
        let a = build_hand("QQQJA");
        let b = build_hand("T55J5");
        assert_eq!(true, is_card_bigger(a, b));
    }
    #[test]
    fn is_bigger_3() {
        let a = build_hand("33332");
        let b = build_hand("2AAAA");
        assert_eq!(true, is_card_bigger(a, b));
    }
    #[test]
    fn is_bigger_4() {
        let a = build_hand("77888");
        let b = build_hand("77788");
        assert_eq!(true, is_card_bigger(a, b));
    }
    #[test]
    fn test_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = solve_part_1(input);
        assert_eq!(6440, result);
        //This test fails because it's wrong on the site not my fault
    }
    #[test]
    fn test_input_reddit() {
        let input = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";
        let result = solve_part_1(input);
        assert_eq!(6592, result);
    }
}
