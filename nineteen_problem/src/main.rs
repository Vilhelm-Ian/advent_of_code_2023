use std::collections::HashMap;
mod input;

#[derive(Debug)]
struct Item {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Item {
    fn new(x: i32, m: i32, a: i32, s: i32) -> Item {
        Item { x, m, a, s }
    }
}

enum Operation<'a> {
    Accept,
    Reject,
    Send(&'a str),
}

fn main() {
    let result = solve(input::INPUT);
    println!("{:?}", result);
}

fn parse_workflows(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut result = HashMap::new();
    input.lines().for_each(|line| {
        let splitted = line.split('{').collect::<Vec<&str>>();
        let workflow: Vec<&str> = splitted[1][0..&splitted[1].len() - 1].split(',').collect();
        result.insert(splitted[0], workflow);
    });
    result
}

fn parse_items(input: &str) -> Vec<Item> {
    input
        .lines()
        .map(|line| {
            let values: Vec<i32> = line[1..line.len() - 1]
                .split(',')
                .map(|value| value[2..].parse::<i32>().unwrap())
                .collect();
            Item::new(values[0], values[1], values[2], values[3])
        })
        .collect()
}

fn evaluate_workflow<'a>(workflow: &Vec<&'a str>, item: &Item) -> Operation<'a> {
    let len = workflow.len();
    for instruction in workflow.iter().take(len - 1) {
        let (did_pass, result) = evaluate_criteria(instruction, item);
        if did_pass {
            println!("matched");
            return match_operation(result);
        }
    }
    match_operation(workflow[len - 1])
}

fn evaluate_criteria<'a>(criteria: &'a str, item: &Item) -> (bool, &'a str) {
    let splitted: Vec<&str> = criteria.split(':').collect();
    let instruction = splitted[0];
    let operation = splitted[1];
    let value = match &instruction[0..1] {
        "a" => item.a,
        "m" => item.m,
        "x" => item.x,
        "s" => item.s,
        _ => panic!("invalid criteria"),
    };
    let is_smaller = match &instruction[1..2] {
        "<" => true,
        ">" => false,
        _ => panic!("invalid comparison operator"),
    };
    let second_value = instruction[2..].parse::<i32>().unwrap();
    if is_smaller {
        if value < second_value {
            return (true, operation);
        }
        if value > second_value {
            return (false, operation);
        }
    }
    println!("{:?}>{:?}", value, second_value);
    if value > second_value {
        (true, operation)
    } else {
        (false, operation)
    }
}

fn match_operation(instruction: &str) -> Operation {
    match instruction {
        "R" => Operation::Reject,
        "A" => Operation::Accept,
        _ => Operation::Send(instruction),
    }
}

fn solve(input: &str) -> i32 {
    let splitted: Vec<&str> = input.split("\n\n").collect();
    let workflows = parse_workflows(splitted[0]);
    let items = parse_items(splitted[1]);
    let mut accepted = vec![];
    for item in items {
        let mut current_workflow = workflows.get("in").unwrap();
        loop {
            match evaluate_workflow(current_workflow, &item) {
                Operation::Accept => {
                    accepted.push(item);
                    break;
                }
                Operation::Reject => {
                    println!("rejected at {:?} {:?}", item, current_workflow);
                    break;
                }
                Operation::Send(destination) => {
                    println!("{:?} {:?}", item, destination);
                    current_workflow = workflows.get(destination).expect("invalid destination");
                }
            };
        }
    }
    accepted
        .iter()
        .map(|item| item.x + item.m + item.a + item.s)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    #[test]
    fn part_1_test() {
        let result = solve(INPUT);
        assert_eq!(result, 19114);
    }
}
