fn main() {
    let part1_input = [[53, 275], [71, 1181], [78, 1215], [80, 1524]];
    let result_part_1: u64 = part1_input
        .iter()
        .map(|x| solve_part_1(x[0], x[1]))
        .product();
    println!("{:?}", result_part_1);
    let part_2_input = part1_input
        .iter()
        .map(|arr| arr.iter().map(|number| number.to_string()).collect())
        .fold(
            [String::from(""), String::from("")],
            |mut acc, arr: Vec<String>| {
                acc[0] = format!("{}{}", acc[0], arr[0]);
                acc[1] = format!("{}{}", acc[1], arr[1]);
                acc
            },
        )
        .iter()
        .map(|num| {
            println!("{num}");
            num.parse().unwrap()
        })
        .collect::<Vec<u64>>();
    let result_part_2 = solve_part_1(53_71_78_80, 275_1181_1215_1524);
    println!("{:?}", result_part_2);

    // solve_part_1(7, 9);
    // solve_part_1(15, 40);
    // solve_part_1(30, 200);
}

fn solve_part_1(time: u64, distance: u64) -> u64 {
    let [biggest, smallest] = find_smallest_and_biggest(time, distance);
    let mut smallest = smallest.floor() as u64;
    let mut biggest = biggest.floor() as u64;

    if !is_solution(smallest, time, distance) {
        smallest += 1;
    }
    if !is_solution(biggest, time, distance) {
        biggest += 1;
    }
    biggest - smallest + 1
}

fn is_solution(input: u64, time: u64, distance: u64) -> bool {
    input * (time - input) > distance
}

fn find_smallest_and_biggest(time: u64, distance: u64) -> [f64; 2] {
    // x * (time - x) = distance + 1
    // -x^2 + x*time = distance+ 1
    // -x^2 + x*time - (distance +1) = 0
    // x= -time +/- \sqrt{time^2-4*-1*(-distance-1)}/-2
    let time = time as f64;
    let distance = distance as f64;
    let first_solution =
        (-1.0 * time - f64::sqrt(time * time - 4.0 * -1.0 * (-distance - 1.0))) / (-2.0);
    let second_solution =
        (-1.0 * time + f64::sqrt(time * time - 4.0 * -1.0 * (-distance - 1.0))) / (-2.0);
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
    fn test_part_1_2() {
        let result = solve_part_1(15, 40);
        assert_eq!(result, 8);
    }
    #[test]
    fn test_part_1_3() {
        let result = solve_part_1(30, 200);
        assert_eq!(result, 9);
    }
    #[test]
    fn test_part_2() {
        let result = solve_part_1(71530, 940200);
        assert_eq!(result, 71503);
    }
}
