mod input;
use rayon::{prelude::*, result};
use std::collections::HashMap;

#[derive(Debug)]
struct Range {
    source_range_start: i64,
    destination_range_start: i64,
    range_length: i64,
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
    fn new(destination_range_start: i64, source_range_start: i64, range_length: i64) -> Range {
        Range {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
}

fn main() {
    let result = solve_part_2(input::INPUT);
    // let result = solve_part_2_fast(input::TEST_INPUT);
    println!("{:?}", result);
}
fn solve_part_1(input: &str) -> i64 {
    let seeds: Vec<i64> = parse_seeds(input.lines().next().expect("seeds not found"));
    let entries = parse_input(input);
    seeds
        .iter()
        .map(|seed| find_for_input_output(*seed, "seed", &entries, "location"))
        .min()
        .expect("no minimum value found because seeds is empty")
}

fn parse_input(input: &str) -> HashMap<&str, Entry> {
    let mut entris = HashMap::new();
    input
        .split("\n\n")
        .skip(1)
        .map(|x| x.split('\n').collect::<Vec<&str>>())
        .for_each(|map| {
            let entry = parse_map(map);
            entris.insert(entry.input, entry);
        });
    entris
}

fn parse_input_2(input: &str) -> Vec<Entry> {
    input
        .split("\n\n")
        .skip(1)
        .map(|x| x.split('\n').collect::<Vec<&str>>())
        .map(|map| parse_map(map))
        .collect()
}

fn parse_map(map: Vec<&str>) -> Entry {
    let mut lines = map.iter();
    let [from, to] = parse_from_to(lines.next().expect("expect invalid map"));
    let mut entry = Entry::new(from, to);
    lines.for_each(|range| entry.ranges.push(parse_range(range)));
    entry.ranges.sort_by_key(|a| a.destination_range_start);
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

fn parse_seeds(line: &str) -> Vec<i64> {
    line.split(' ')
        .skip(1)
        .map(|number| number.parse().expect("invalid seed number"))
        .collect()
}

fn convert_from_to(from: i64, ranges: &Vec<Range>) -> i64 {
    for range in ranges {
        if (range.source_range_start..range.source_range_start + range.range_length).contains(&from)
        {
            return (from - range.source_range_start) + range.destination_range_start;
        }
    }
    from
}

fn solve_part_2(input: &str) -> i64 {
    let seed_ranges: Vec<[i64; 2]> =
        parse_seeds_part_2(input.lines().next().expect("seeds not found"));
    let mut entries = parse_input(input);
    seed_ranges.iter().fold(i64::MAX, |min, range| {
        let result = (range[0]..range[1])
            .collect::<Vec<i64>>()
            .par_iter()
            .fold(
                || i64::MAX,
                |min, seed| {
                    let res = find_for_input_output(*seed, "seed", &entries, "location");
                    if res < min {
                        res
                    } else {
                        min
                    }
                },
            )
            .reduce(
                || i64::MAX,
                |min, res| {
                    if res < min {
                        res
                    } else {
                        min
                    }
                },
            );
        if result < min {
            result
        } else {
            min
        }
    })
}

fn parse_seeds_part_2(line: &str) -> Vec<[i64; 2]> {
    line.split(' ')
        .skip(1)
        .map(|number| number.parse().expect("seed is not valid number"))
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|chunk| [chunk[0], chunk[0] + chunk[1]])
        .collect()
}

fn find_for_input_output(
    input_value: i64,
    input_name: &str,
    entries: &HashMap<&str, Entry>,
    output_name: &str,
) -> i64 {
    if output_name == input_name {
        return input_value;
    }
    let entry = entries
        .get(input_name)
        .expect("didn't find entry {input_name}");
    let output_value = convert_from_to(input_value, &entry.ranges);
    find_for_input_output(output_value, entry.output, entries, output_name)
}

fn parse_seeds_part_2_fast(line: &str) -> Vec<i64> {
    let mut result = vec![];
    line.split(' ')
        .skip(1)
        .map(|number| number.parse().expect("seed is not valid number"))
        .collect::<Vec<i64>>()
        .chunks(2)
        .for_each(|chunk| {
            result.push(chunk[0]);
            result.push(chunk[0] + chunk[1]);
        });
    result
}

fn solve_part_2_fast(input: &str) -> i64 {
    let mut first_line = input.lines();
    let seed_ranges = parse_seeds_part_2_fast(first_line.next().expect("no first line"));
    println!("ranges {:?}", seed_ranges);
    let mut entries = parse_input_2(input);
    let mut current_indexes = seed_ranges.clone();
    for entry in entries {
        println!("{:?}", current_indexes);
        let mut i = 0;
        let mut temp_indexes = vec![];
        while i + 1 < current_indexes.len() {
            println!(
                "finding sub indexes of {:?} {:?} {:?}",
                current_indexes[i],
                current_indexes[i + 1],
                &entry
            );
            let mut indexes = find_sub_indexes(current_indexes[i], current_indexes[i + 1], &entry);
            println!("found index {:?}", indexes);
            temp_indexes.append(&mut indexes);
            i += 2;
        }
        current_indexes = temp_indexes;
    }

    *current_indexes
        .iter()
        .min()
        .expect("current indexes is empty ")
}

fn find_sub_indexes_improved(start: i64, end: i64, entry: &Entry) -> Vec<[i64; 2]> {
    let mut indexes = vec![start, end];
    let mut indexes_solution = vec![];
    let indexes_clone = indexes.clone();
    for range in entry.ranges.iter() {
        let end_of_entry_range = range.source_range_start + range.range_length;
        if (range.source_range_start..end_of_entry_range).contains(&start) {
            if (range.source_range_start..end_of_entry_range).contains(&end) {
                return vec![[start, end]];
            } else {
                indexes_solution.push([start, end_of_entry_range - 1]);
                let mut other_index = find_sub_indexes_improved(end_of_entry_range, end, entry);
                indexes_solution.append(&mut other_index);
            }
        }
    }
    if indexes_solution.is_empty() {
        indexes_solution.push([start, end]);
    }
    indexes_solution
}

fn convert_sub_indexes(sub_indexes: Vec<[i64; 2]>, entry: &Entry) -> Vec<[i64; 2]> {
    let mut solution = vec![];
    for sub_index in sub_indexes {
        let mut temp = [-1, -1];
        for range in entry.ranges.iter() {
            let update_value = range.destination_range_start - range.source_range_start;
            let end_of_entry_range = range.source_range_start + range.range_length;
            if (range.source_range_start..end_of_entry_range).contains(&sub_index[0]) {
                temp[0] = sub_index[0] + update_value;
            }
            if (range.source_range_start..end_of_entry_range).contains(&sub_index[1]) {
                temp[1] = sub_index[1] + update_value;
            }
        }
        if temp[0] == -1 {
            temp = [sub_index[0], sub_index[1]];
        }
        if temp[1] == -1 || temp[0] == -1 {
            panic!("couldn't convert");
        }
        solution.push(temp);
    }
    solution
}

fn solve_part_2_improved(input: &str) -> i64 {
    let first_line = input.lines().collect::<Vec<&str>>()[0];
    let seeds = parse_seeds_part_2(first_line);
    let entries = parse_input_2(input);
    let indexes = solve_with_recursion(seeds, &entries, 0);
    let result = indexes.iter().map(|index| index[0]);

    result.min().expect("didn't find min")
}

fn solve_with_recursion(indexes: Vec<[i64; 2]>, entries: &Vec<Entry>, i: usize) -> Vec<[i64; 2]> {
    match i < entries.len() {
        true => {
            let mut new_indexes = vec![];
            for index in indexes.iter() {
                let name_later = find_sub_indexes_improved(index[0], index[1], &entries[i]);
                let mut converted = convert_sub_indexes(name_later, &entries[i]);
                new_indexes.append(&mut converted);
            }
            solve_with_recursion(new_indexes, entries, i + 1)
        }
        false => indexes,
    }
}

fn find_sub_indexes(start: i64, end: i64, entry: &Entry) -> Vec<i64> {
    let mut indexes = vec![start, end];
    let mut indexes_solution = vec![];
    let indexes_clone = indexes.clone();
    let mut solution = vec![];
    for (i, index) in indexes.iter_mut().enumerate() {
        for range in entry.ranges.iter() {
            let end_of_entry_range = range.source_range_start + range.range_length;
            if (range.source_range_start..=end_of_entry_range).contains(&index) {
                if indexes_clone.len() != i + 1
                    && !(range.source_range_start..=end_of_entry_range)
                        .contains(&indexes_clone[i + 1])
                {
                    if !indexes_solution.contains(&end_of_entry_range) {
                        indexes_solution.push(end_of_entry_range)
                    }
                }
            }
            if !indexes_solution.contains(index) {
                indexes_solution.push(*index)
            }
        }
    }
    let mut offset = 0;
    let mut index_solution_clone = indexes_solution.clone();
    for (i, index) in index_solution_clone.iter_mut().enumerate() {
        for range in entry.ranges.iter() {
            let update_value = range.destination_range_start - range.source_range_start;
            let end_of_entry_range = range.source_range_start + range.range_length;
            if (range.source_range_start..=end_of_entry_range).contains(index) {
                solution.push(*index + update_value);
                if offset > i {
                    continue;
                }
                indexes_solution.remove(i - offset);
                offset += 1;
            }
        }
    }
    solution.append(&mut indexes_solution);
    solution
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
    #[test]
    fn parse_seeds_part_2_test() {
        let input = "seeds: 79 14 55 13";
        let result = parse_seeds_part_2(input);
        assert_eq!(result, vec![[79, 93], [55, 68]]);
    }
    #[test]
    fn test_input_2() {
        let result = solve_part_2(input::TEST_INPUT);
        assert_eq!(result, 46);
    }
    #[test]
    fn test_input_2_one_seed_2() {
        let input = "seeds: 79 14

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = solve_part_2(input);
        assert_eq!(result, 46);
    }

    #[test]
    fn test_input_2__secondseed_2() {
        let input = "seeds: 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = solve_part_2(input);
        assert_eq!(result, 56);
    }

    //     #[test]
    //     fn new_find_sub_indexes() {
    //         let input = "light-to-temperature map:
    // 45 77 23
    // 81 45 19
    // 68 64 13"
    //             .lines()
    //             .collect::<Vec<&str>>();
    //         let entry = parse_map(input);
    //         let result = find_sub_indexes(74, 88, &entry);
    //         assert_eq!(result, vec![78, 81, 45, 56])
    //     }
    #[test]
    fn new_find_sub_indexes_2() {
        let input = "seed-to-soil map:
50 98 2
52 50 48"
            .lines()
            .collect::<Vec<&str>>();
        let entry = parse_map(input);
        let result = find_sub_indexes(79, 93, &entry);
        assert_eq!(result, vec![81, 95])
    }
    #[test]
    fn new_find_sub_indexes_3() {
        let input = "soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15"
            .lines()
            .collect::<Vec<&str>>();
        let entry = parse_map(input);
        let result = find_sub_indexes(81, 95, &entry);
        assert_eq!(result, vec![81, 95])
    }

    #[test]
    fn solution_part_2_new_find_sub_indexes() {
        let solution = solve_part_2_fast(input::TEST_INPUT);
        assert_eq!(solution, 46)
    }
    #[test]
    fn new_find_sub_indexes_improved() {
        let input = "light-to-temperature map:
45 77 23
81 45 19
68 64 13"
            .lines()
            .collect::<Vec<&str>>();
        let entry = parse_map(input);
        let result = find_sub_indexes_improved(74, 88, &entry);
        assert_eq!(result, vec![[74, 76], [77, 88]])
    }
    #[test]
    fn new_find_sub_indexes_improved_2() {
        let input = "seed-to-soil map:
50 98 2
52 50 48"
            .lines()
            .collect::<Vec<&str>>();
        let entry = parse_map(input);
        let result = find_sub_indexes_improved(79, 93, &entry);
        assert_eq!(result, vec![[79, 93]])
    }
    #[test]
    fn new_find_sub_indexes_improved_3() {
        let input = "soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15"
            .lines()
            .collect::<Vec<&str>>();
        let entry = parse_map(input);
        let result = find_sub_indexes_improved(81, 95, &entry);
        assert_eq!(result, vec![[81, 95]])
    }
    #[test]
    fn convert_indexes_test() {
        let input = "light-to-temperature map:
45 77 23
81 45 19
68 64 13"
            .lines()
            .collect::<Vec<&str>>();
        let entry = parse_map(input);
        let indexes = find_sub_indexes_improved(74, 88, &entry);
        println!("{:?}", indexes);
        let result = convert_sub_indexes(indexes, &entry);
        assert_eq!(result, vec![[78, 80], [45, 56]])
    }
    #[test]
    fn convert_indexes_2() {
        let input = "seed-to-soil map:
50 98 2
52 50 48"
            .lines()
            .collect::<Vec<&str>>();
        let entry = parse_map(input);
        let indexes = find_sub_indexes_improved(79, 93, &entry);
        let result = convert_sub_indexes(indexes, &entry);
        assert_eq!(result, vec![[81, 95]])
    }
    #[test]
    fn convert_indexes_3() {
        let input = "soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15"
            .lines()
            .collect::<Vec<&str>>();
        let entry = parse_map(input);
        let indexes = find_sub_indexes_improved(81, 95, &entry);
        let result = convert_sub_indexes(indexes, &entry);
        assert_eq!(result, vec![[81, 95]])
    }
}
