mod input;

fn main() {
    let result = solve_part_1(input::INPUT);
    println!("{:?}", result);
    let result = solve_part_2(input::INPUT);
    println!("{:?}", result);
}

fn generate_pyramid(line: &str) -> Vec<Vec<i32>> {
    let first_line = line
        .split(' ')
        .map(|number| number.parse().expect("couldn't convert strto number"))
        .collect::<Vec<i32>>();
    let mut pyramid = vec![first_line];
    while !is_last_element_all_zeroes(&pyramid) {
        let last_level = pyramid.len() - 1;
        let new_level = (0..pyramid[last_level].len() - 1)
            .map(|i| pyramid[last_level][i + 1] - pyramid[last_level][i])
            .collect();
        pyramid.push(new_level);
    }
    pyramid
}

fn is_last_element_all_zeroes(pyramid: &Vec<Vec<i32>>) -> bool {
    pyramid[pyramid.len() - 1].iter().all(|num| num == &0)
}

fn add_place_holders(mut pyramid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    (1..pyramid.len()).rev().for_each(|i| {
        let level_length = pyramid[i].len();
        let place_holder = pyramid[i][level_length - 1] + pyramid[i - 1][level_length - 1];
        pyramid[i - 1].push(place_holder)
    });
    pyramid
}

fn add_place_holders_backwards(mut pyramid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    (1..pyramid.len()).rev().for_each(|i| {
        let level_length = pyramid[i].len();
        let place_holder = pyramid[i - 1][0] - pyramid[i][0];
        pyramid[i - 1].insert(0, place_holder)
    });
    pyramid
}

fn solve_part_1(input: &str) -> i32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| generate_pyramid(line))
        .map(add_place_holders)
        .map(|pyramid| *pyramid[0].last().unwrap())
        .sum()
}

fn solve_part_2(input: &str) -> i32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| generate_pyramid(line))
        .map(add_place_holders_backwards)
        .map(|pyramid| pyramid[0][0])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_pyramid_1() {
        let line = input::TEST_INPUT.lines().collect::<Vec<&str>>()[0];
        let result = generate_pyramid(line);
        let expected = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(result, expected);
    }
    #[test]
    fn generate_pyramid_2() {
        let line = input::TEST_INPUT.lines().collect::<Vec<&str>>()[1];
        let result = generate_pyramid(line);
        let expected = vec![
            vec![1, 3, 6, 10, 15, 21],
            vec![2, 3, 4, 5, 6],
            vec![1, 1, 1, 1],
            vec![0, 0, 0],
        ];
        assert_eq!(result, expected);
    }
    #[test]
    fn generate_pyramid_3() {
        let line = input::TEST_INPUT.lines().collect::<Vec<&str>>()[2];
        let result = generate_pyramid(line);
        let expected = vec![
            vec![10, 13, 16, 21, 30, 45],
            vec![3, 3, 5, 9, 15],
            vec![0, 2, 4, 6],
            vec![2, 2, 2],
            vec![0, 0],
        ];
        assert_eq!(result, expected);
    }
    #[test]
    fn place_holder_1() {
        let line = input::TEST_INPUT.lines().collect::<Vec<&str>>()[0];
        let pyramid = generate_pyramid(line);
        let mut result = add_place_holders(pyramid);
        assert_eq!(result[0].pop().unwrap(), 18);
    }
    #[test]
    fn place_holder_2() {
        let line = input::TEST_INPUT.lines().collect::<Vec<&str>>()[1];
        let pyramid = generate_pyramid(line);
        let mut result = add_place_holders(pyramid);
        assert_eq!(result[0].pop().unwrap(), 28);
    }
    #[test]
    fn place_holder_3() {
        let line = input::TEST_INPUT.lines().collect::<Vec<&str>>()[2];
        let pyramid = generate_pyramid(line);
        let mut result = add_place_holders(pyramid);
        assert_eq!(result[0].pop().unwrap(), 68);
    }
    #[test]
    fn part_1_test() {
        assert_eq!(solve_part_1(input::TEST_INPUT), 114);
    }
    #[test]
    fn part_2_test() {
        assert_eq!(solve_part_2(input::TEST_INPUT), 2);
    }
}
