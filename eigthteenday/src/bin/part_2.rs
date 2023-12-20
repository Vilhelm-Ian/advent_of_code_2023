mod input;

use std::collections::HashSet;

fn main() {
    let result = solve(input::INPUT);
    println!("{:?}", result);
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .collect()
}

fn get_edges(instructions: Vec<Vec<&str>>) -> Vec<(i64, i64)> {
    let mut edges = vec![];
    let mut current_location = (1, 1);
    for instruction in instructions {
        edges.push(current_location);
        let distance =
            i64::from_str_radix(&instruction[2][2..instruction[2].len() - 2], 16).unwrap();
        let direction = &instruction[2][instruction[2].len() - 2..instruction[2].len() - 1];
        println!("{:?}", distance);
        match direction {
            "0" => current_location.1 += distance,
            "2" => current_location.1 -= distance,
            "1" => current_location.0 -= distance,
            "3" => current_location.0 += distance,
            _ => panic!("Invalid direction"),
        }
    }

    // let current_length = 1;
    // let current_width = 1;
    // let mut current_location = (1, 1);
    // let mut edges = vec![];
    // for instruction in instructions {
    //     edges.push(current_location);
    //     let distance = instruction[1].parse::<i64>().unwrap();
    //     match instruction[0] {
    //         "R" => current_location.1 += distance,
    //         "L" => current_location.1 -= distance,
    //         "D" => current_location.0 -= distance,
    //         "U" => current_location.0 += distance,
    //         _ => panic!("Invalid direction"),
    //     }
    // }
    edges
}

fn calculate_area(a: &(i64, i64), b: &(i64, i64)) -> f64 {
    let average_height = (a.0 + b.0) as f64 / 2.0;
    let width = b.1 - a.1;
    width as f64 * average_height
}

fn caculate_perimeter(a: &(i64, i64), b: &(i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn solve(input: &str) -> i64 {
    let instructions = parse(input);
    let mut edges = get_edges(instructions);
    let perimeter = edges
        .iter()
        .enumerate()
        .map(|(index, edge)| {
            if index < edges.len() - 1 {
                caculate_perimeter(edge, &edges[index + 1])
            } else {
                caculate_perimeter(edge, &edges[0])
            }
        })
        .sum::<i64>();
    println!(" the perimeter is {:?}", perimeter);
    let area_under_edges = edges
        .iter()
        .enumerate()
        .map(|(index, edge)| {
            if index < edges.len() - 1 {
                calculate_area(edge, &edges[index + 1])
            } else {
                calculate_area(edge, &edges[0])
            }
        })
        .sum::<f64>();
    println!("the area under the edges is {:?}", area_under_edges);
    area_under_edges as i64 + (perimeter / 2) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    pub const OUTLINE: &str = "#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######";
    #[test]
    fn test_1() {
        let result = solve(TEST_INPUT);
        assert_eq!(result, 952408144115);
    }
}
