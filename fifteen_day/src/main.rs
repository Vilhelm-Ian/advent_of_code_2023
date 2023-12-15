mod input;
mod part_1;
mod part_2;

fn main() {
    let result = part_1::sum_sequence(input::INPUT);

    println!("{result}");
    let result = part_2::solve(input::INPUT);

    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hash_algorithm_1() {
        let input = "HASH";
        let result = part_1::hash_algorithm(input);
        assert_eq!(result, 52);
    }
    #[test]
    fn sum_sequence_1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = part_1::sum_sequence(input);
        assert_eq!(result, 1320);
    }
    #[test]
    fn part_2_test() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = part_2::solve(input);
        assert_eq!(result, 145);
    }
}
