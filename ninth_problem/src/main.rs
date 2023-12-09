mod input;

fn main() {
    println!("Hello, world!");
}

fn generate_pyramid(line: &str) -> Vec<Vec<i32>> {
    vec![]
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
}
