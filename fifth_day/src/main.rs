mod input;

#[derive(Debug)]
struct Range {
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64,
}

#[derive(Debug)]
struct Entry<'a, 'b> {
    input: &'a str,
    output: &'b str,
    ranges: Vec<Range>,
}

impl<'a, 'b> Entry<'a, 'b> {
    fn new(input: &'a str, output: &'b str) -> Entry<'a, 'b> {
        Entry {
            input,
            output,
            ranges: vec![],
        }
    }
}

impl Range {
    fn new(destination_range_start: u64, source_range_start: u64, range_length: u64) -> Range {
        Range {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
}

fn main() {
    let result = solve_part_1(input::INPUT);
    println!("{:?}", result);
}

fn solve_part_1(input: &str) -> u64 {
    let seeds: Vec<u64> = parse_seeds(input.lines().next().expect("seeds not found"));
    let entries = parse_input(input);
    seeds
        .iter()
        .map(|seed| find_for_input_output(*seed, "seed", &entries, "location"))
        .min()
        .expect("no minimum value found because seeds is empty")
}

fn parse_input(input: &str) -> Vec<Entry> {
    let mut entris = vec![];
    input
        .split("\n\n")
        .skip(1)
        .map(|x| x.split('\n').collect::<Vec<&str>>())
        .for_each(|map| entris.push(parse_map(map)));
    entris
}

fn parse_map(map: Vec<&str>) -> Entry {
    let mut lines = map.iter();
    let [from, to] = parse_from_to(lines.next().expect("expect invalid map"));
    let mut entry = Entry::new(from, to);
    lines.for_each(|range| entry.ranges.push(parse_range(range)));
    entry
}

fn parse_from_to(describor: &str) -> [&str; 2] {
    let mut from_to = describor
        .split(' ')
        .next()
        .expect("invalid from-to")
        .split("-to-");
    [
        from_to.next().expect("invalid from"),
        from_to.next().expect("invalid to"),
    ]
}

fn parse_range(range: &str) -> Range {
    let mut ranges = range.split(' ');
    let target_range_start = ranges
        .next()
        .expect("invalid target range")
        .parse()
        .expect("couldn't parse target range");
    let source_range_start = ranges
        .next()
        .expect("invalid source range")
        .parse()
        .expect("couldn't parse source range");
    let range_length = ranges
        .next()
        .expect("invalid range length")
        .parse()
        .expect("couldn't parse range length");
    Range::new(target_range_start, source_range_start, range_length)
}

fn parse_seeds(line: &str) -> Vec<u64> {
    line.split(' ')
        .skip(1)
        .map(|number| number.parse().expect("invalid seed number"))
        .collect()
}

fn convert_from_to(from: u64, ranges: &Vec<Range>) -> u64 {
    for range in ranges {
        if (range.source_range_start..range.source_range_start + range.range_length).contains(&from)
        {
            return (from - range.source_range_start) + range.destination_range_start;
        }
    }
    from
}

fn find_for_input_output(
    input_value: u64,
    input_name: &str,
    entries: &Vec<Entry>,
    output_name: &str,
) -> u64 {
    let mut result = None;
    for entry in entries {
        if entry.input == input_name {
            let output_value = convert_from_to(input_value, &entry.ranges);
            result = Some(find_for_input_output(
                output_value,
                entry.output,
                entries,
                output_name,
            ));
        } else if output_name == input_name {
            return input_value;
        }
    }

    let error = format!("couldn't finnd output value for input {:?}", input_name);
    result.expect(&error)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_1() {
        let result = solve_part_1(input::TEST_INPUT);
        assert_eq!(result, 35);
    }
    #[test]
    fn seed_to_soil() {
        let entries = parse_input(input::TEST_INPUT);
        let result = find_for_input_output(79, "seed", &entries, "location");
        assert_eq!(result, 82);
    }
    #[test]
    fn convert_test_1() {
        let range_1 = parse_range("50 98 2");
        let range_2 = parse_range("52 50 48");
        let ranges = vec![range_1, range_2];
        let result = convert_from_to(99, &ranges);
        assert_eq!(result, 51);
    }
    #[test]
    fn convert_test_2() {
        let range_1 = parse_range("50 98 2");
        let range_2 = parse_range("52 50 48");
        let ranges = vec![range_1, range_2];
        let result = convert_from_to(50, &ranges);
        assert_eq!(result, 52);
    }
    #[test]
    fn convert_test_3() {
        let range_1 = parse_range("50 98 2");
        let range_2 = parse_range("52 50 48");
        let ranges = vec![range_1, range_2];
        let result = convert_from_to(1, &ranges);
        assert_eq!(result, 1);
    }
    #[test]
    fn convert_test_4() {
        let range_1 = parse_range("60 56 37");
        let range_2 = parse_range("56 93 4");
        let ranges = vec![range_1, range_2];
        let result = convert_from_to(77, &ranges);
        assert_eq!(result, 81);
    }
}
