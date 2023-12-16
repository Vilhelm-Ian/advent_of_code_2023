mod input;
mod part_1;
mod part_2;

fn main() {
    let result = part_1::solve(input::INPUT);
    println!("{result}");
    let result = part_2::solve(input::INPUT);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
    use super::*;
    #[test]
    fn test_1() {
        let result = part_1::solve(TEST_INPUT);
        assert_eq!(result, 46);
    }
    #[test]
    fn horizontal_ranges() {
        let grid = part_2::parse(TEST_INPUT);
        let result = part_2::find_horizontal_ranges(&grid);
        assert_eq!(result[result.len() - 1], [[9, 5], [9, 9]]);
        assert_eq!(result[0], [[0, 0], [0, 1]]);
        assert_eq!(result[1], [[0, 1], [0, 5]]);
        assert_eq!(result[2], [[0, 5], [0, 9]]);
        assert_eq!(result[3], [[1, 0], [1, 4]]);
    }
    // #[test]
    // fn verticaal_ranges() {
    //     let grid = part_2::parse(TEST_INPUT);
    //     let result = part_2::find_vertical_ranges(&grid);
    //     assert_eq!(result[result.len() - 1], [[8, 9], [9, 9]]);
    //     assert_eq!(result[0], [[0, 0], [9, 0]]);
    //     assert_eq!(result[1], [[0, 5], [9, 5]]);
    //     assert_eq!(result[2], [[1, 2], [9, 2]]);
    //     assert_eq!(result[3], [[4, 2], [4, 4]]);
    // }
    // #[test]
    fn test_2() {
        let result = part_2::solve(TEST_INPUT);
        assert_eq!(result, 51);
    }
}
