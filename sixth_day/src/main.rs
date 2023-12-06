fn main() {
    // solve_part_1(7, 9);
    // solve_part_1(15, 40);
    // solve_part_1(30, 200);
}

fn solve_part_1(time: i32, distance: i32) -> i32 {
    let [biggest, smallest] = find_smallest_and_biggest(time, distance);
    let mut smallest = smallest.floor() as i32;
    let mut biggest = biggest.floor() as i32;

    if !is_solution(smallest, time, distance) {
        smallest += 1;
    }
    if !is_solution(biggest, time, distance) {
        biggest += 1;
    }
    biggest - smallest + 1
}

fn is_solution(input: i32, time: i32, distance: i32) -> bool {
    input * (time - input) > distance
}

fn find_smallest_and_biggest(time: i32, distance: i32) -> [f32; 2] {
    // x * (time - x) = distance + 1
    // -x^2 + x*time = distance+ 1
    // -x^2 + x*time - (distance +1) = 0
    // x= -time +/- \sqrt{time^2-4*-1*(-distance-1)}/-2
    let time = time as f32;
    let distance = distance as f32;
    let first_solution =
        (-1.0 * time - f32::sqrt(time * time - 4.0 * -1.0 * (-distance - 1.0))) / (-2.0);
    let second_solution =
        (-1.0 * time + f32::sqrt(time * time - 4.0 * -1.0 * (-distance - 1.0))) / (-2.0);
    [first_solution, second_solution]
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_part_1() {
        let result = solve_part_1(7, 9);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_part_2() {
        let result = solve_part_1(15, 40);
        assert_eq!(result, 8);
    }
    #[test]
    fn test_part_3() {
        let result = solve_part_1(30, 200);
        assert_eq!(result, 9);
    }
}
