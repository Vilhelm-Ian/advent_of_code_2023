mod input;

fn main() {
    println!("Hello, world!");
}

fn solve_part_1(input: &str) -> i32 {
    let mut numbers = vec![];
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for y in 0..lines.len() {
        let mut number = String::from("");
        let mut is_part_number = false;
        for x in 0..lines[y].len() {
            // println!("{:?}", lines[y][x]);
            if lines[y][x].is_ascii_digit() {
                number.push(lines[y][x]);
                if is_part_digit(&lines, y, x) {
                    //println!("{parsed_number}");
                    is_part_number = true;
                };
            } else {
                if let Ok(parsed_number) = number.parse::<i32>() {
                    if is_part_number {
                        numbers.push(parsed_number);
                    }
                }
                is_part_number = false;
                number = String::from("");
            }
        }
    }
    numbers.iter().sum()
}

fn is_part_digit(map: &Vec<Vec<char>>, row: usize, collum: usize) -> bool {
    // println!("{row} {collum}");
    for y in 0..3 {
        let y = y - 1;
        if (y + row as i32) == -1 || (y + row as i32) == map.len() as i32 {
            continue;
        }
        for x in 0..3 {
            let x = x - 1;
            if (x + collum as i32) == -1 || (x + collum as i32) == map[0].len() as i32 {
                continue;
            }
            if y == 0 && x == 0 {
                continue;
            }
            let row = (row as i32 + y) as usize;
            let collum = (collum as i32 + x) as usize;
            println!("value {:?}", map[row][collum]);
            // println!("row{row} ocllum {collum} y {y} x {x}");
            match map[row][collum] {
                '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => (),
                _ => {
                    return true;
                }
            };
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_test_input() {
        let result = solve_part_1(input::TEST_INPUT_PART_1);
        assert_eq!(result, 4361);
    }
}
