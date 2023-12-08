use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::str::Chars;
mod input;

struct Node<'a, 'b> {
    left: &'a str,
    right: &'b str,
}

impl<'a, 'b> Node<'a, 'b> {
    fn new(left: &'a str, right: &'b str) -> Node<'a, 'b> {
        Node { left, right }
    }
}

fn main() {
    let result = solve_part_1(input::INPUT);
    println!("{:?}", result);
    let result = solve_part_2(input::INPUT);
    println!("{:?}", result);
}

fn parse_nodes(input: &str) -> Vec<[String; 3]> {
    input
        .lines()
        .map(|line| {
            let line = line.replace([',', '(', ')', '='], "");
            let splitted = line.split(' ').collect::<Vec<&str>>();
            [
                splitted[0].to_string(),
                splitted[2].to_string(),
                splitted[3].to_string(),
            ]
        })
        .collect()
}

fn create_hash_map(nodes: &[[String; 3]]) -> HashMap<&String, Node> {
    let mut hash_map = HashMap::new();
    nodes.iter().for_each(|[node, left, right]| {
        hash_map.insert(node, Node::new(left, right));
    });
    hash_map
}

fn solve_part_1(input: &str) -> u64 {
    let mut splitted = input.split("\n\n");
    let instructions: Vec<char> = splitted
        .next()
        .expect("couldn't get instructions")
        .chars()
        .collect();
    let nodes = parse_nodes(splitted.next().unwrap());
    let map = create_hash_map(&nodes);
    let mut i = 0;
    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        for instruction in instructions.iter() {
            current_node = get_next_node(instruction, &map, current_node);
            i += 1;
        }
    }
    i
}

fn solve_part_2(input: &str) -> u64 {
    let mut splitted = input.split("\n\n");
    let instructions: Vec<char> = splitted
        .next()
        .expect("couldn't get instructions")
        .chars()
        .collect();
    let nodes = parse_nodes(splitted.next().unwrap());
    let nodes_clone = nodes.clone();
    let start_nodes = nodes_clone
        .into_iter()
        .filter(|x| x[0].ends_with('A'))
        .collect();
    let map = create_hash_map(&nodes);
    let cycle_lengths = get_cycles(instructions, &start_nodes, &map);
    println!("{:?}", cycle_lengths);
    least_common_multiple(cycle_lengths)
}

fn get_cycles(
    instructions: Vec<char>,
    nodes: &Vec<[String; 3]>,
    map: &HashMap<&String, Node>,
) -> Vec<u64> {
    let mut result = Vec::new();
    for node in nodes {
        result.push(get_cycle_length(&instructions, node[0].as_str(), map));
    }
    result
}

fn get_cycle_length<'a>(
    instructions: &'a Vec<char>,
    mut node: &'a str,
    map: &'a HashMap<&String, Node>,
) -> u64 {
    let mut i = 0;
    loop {
        for instruction in instructions.iter() {
            node = get_next_node(instruction, map, node);
            i += 1;
            if node.ends_with('Z') {
                return i;
            };
        }
    }
}

fn do_all_end_with_z(nodes: &Vec<String>) -> bool {
    let result = nodes.par_iter().try_for_each(|node| {
        if !node.ends_with('Z') {
            return None;
        };
        Some(())
    });
    result.is_some()
}

fn get_next_node<'a>(
    instruction: &char,
    map: &'a HashMap<&String, Node>,
    current_node: &str,
) -> &'a str {
    match instruction {
        'R' => {
            map.get(&current_node.to_string())
                .unwrap_or_else(|| panic!("couldn't find node {current_node}"))
                .right
        }
        'L' => {
            map.get(&current_node.to_string())
                .unwrap_or_else(|| panic!("couldn't find node {current_node}"))
                .left
        }
        _ => panic!("invalid instruction"),
    }
}

fn least_common_multiple(mut numbers: Vec<u64>) -> u64 {
    let mut prime_factors = HashSet::new();
    for number in numbers {
        for prime in find_prime_factors(number) {
            prime_factors.insert(prime);
        }
    }
    prime_factors.iter().product()
}

fn find_prime_factors(mut number: u64) -> Vec<u64> {
    let mut prime_factors = vec![];

    for divisor in 2..=((number as f64).sqrt()) as u64 {
        if is_prime(divisor) {
            while number % divisor == 0 {
                if !prime_factors.contains(&divisor) {
                    prime_factors.push(divisor);
                }
                number /= divisor;
            }
        }
    }
    prime_factors.push(number);
    prime_factors
}

fn is_prime(number: u64) -> bool {
    if number == 1 || number == 2 {
        return true;
    }
    if number % 2 == 0 {
        return false;
    }
    for i in 2..=((number as f64).sqrt()) as u64 {
        if number % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]

mod tests {
    use crate::do_all_end_with_z;

    use super::*;
    #[test]
    fn part_1_test_input_1() {
        let result = solve_part_1(input::TEST_INPUT);
        assert_eq!(result, 2);
    }
    #[test]
    fn part_1_test_input_2() {
        let result = solve_part_1(input::TEST_INPUT_2);
        assert_eq!(result, 6);
    }
    #[test]
    fn do_all_end_with_z_test() {
        let input = ["11Z", "2ZZ"];
        let input = input.iter().map(|x| x.to_string()).collect();
        let result = do_all_end_with_z(&input);
        assert!(result);
    }
    #[test]
    fn part_2_test_input() {
        let result = solve_part_2(input::TEST_INPUT_PART_2);
        assert_eq!(result, 6);
    }
}
