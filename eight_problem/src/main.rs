use rayon::prelude::*;
use std::collections::HashMap;
use std::str::Chars;
mod input;

struct Node<'a, 'b> {
    left: &'a str,
    right: &'b str,
    // distance_from_z: Option<i32>,
}

impl<'a, 'b> Node<'a, 'b> {
    fn new(left: &'a str, right: &'b str) -> Node<'a, 'b> {
        // let mut distance_from_z = None;
        // if left == "ZZZ" || right == "ZZZ" {
        //     distance_from_z = Some(1)
        // }
        Node {
            left,
            right,
            // distance_from_z,
        }
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

fn create_hash_map<'a>(nodes: &'a Vec<[String; 3]>) -> HashMap<&'a String, Node> {
    let mut hash_map = HashMap::new();
    nodes.iter().for_each(|[node, left, right]| {
        hash_map.insert(node, Node::new(left, right));
    });
    hash_map
}

fn solve_part_1(input: &str) -> i32 {
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

fn solve_part_2(input: &str) -> i32 {
    let mut splitted = input.split("\n\n");
    let instructions: Vec<char> = splitted
        .next()
        .expect("couldn't get instructions")
        .chars()
        .collect();
    let nodes = parse_nodes(splitted.next().unwrap());
    let map = create_hash_map(&nodes);
    let mut i = 0;
    let mut current_nodes: Vec<String> = nodes
        .iter()
        .filter_map(|node| {
            if node[0].ends_with('A') {
                Some(node[0].clone())
            } else {
                None
            }
        })
        .collect();
    loop {
        for instruction in instructions.iter() {
            current_nodes = current_nodes
                .par_iter()
                .map(|node| get_next_node(instruction, &map, node).to_string())
                .collect();
            i += 1;
            if do_all_end_with_z(&current_nodes) {
                return i;
            };
        }
    }
}

fn do_all_end_with_z(nodes: &Vec<String>) -> bool {
    let node_that_end_with_z: Vec<&String> = nodes
        .par_iter()
        .filter(|node| node.ends_with('Z'))
        .collect();
    node_that_end_with_z.len() == nodes.len()
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
