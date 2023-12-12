use rayon::prelude::*;
use std::collections::HashSet;

mod input;

fn main() {
    let result = how_many_arragments(input::INPUT);
    println!("{:?}", result);
}

fn how_many_arragments(input: &str) -> usize {
    let rows = parse_input(input);
    // let length = rows.len();
    // println!("{:?}", length);
    // let result = rows.iter().enumerate().fold(0, |acc, (i, row)| {
    //     println!("{:?}", i);
    //     let values: Vec<usize> = row[1]
    //         .split(',')
    //         .map(|x| x.parse::<usize>().unwrap())
    //         .collect();
    //     let mut set = HashSet::new();
    //     acc + insert_all(row[0], &values.clone(), values, &mut set)
    // });
    let result = rows
        .par_iter()
        .fold(
            || 0,
            |acc, row| {
                let values: Vec<usize> = row[1]
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                let mut set = HashSet::new();
                let mut seen = HashSet::new();
                acc + insert_all(row[0], &values.clone(), values, &mut set, &mut seen)
            },
        )
        .reduce(|| 0, |current, acc| current + acc);
    result
}

fn insert_all(
    springs: &str,
    original_records: &Vec<usize>,
    records: Vec<usize>,
    set: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) -> usize {
    if seen.get(&springs.to_string()).is_some() {
        return 0;
    }
    seen.insert(springs.to_string());
    // println!("{:?}", springs);
    let unknows = all_unknwons(springs);
    let mut result = springs.to_string();
    let broken_in_string = springs.chars().filter(|x| x == &'#').collect::<Vec<char>>();
    if broken_in_string.len() == original_records.iter().sum()
        && how_many_broken_sequences(springs) == original_records.len()
    {
        if !unknows.is_empty() {
            result = springs.replace('?', ".");
        }
        set.insert(result);
        return 0;
    }
    let mut seen_record = HashSet::new();
    for record in records.iter() {
        if seen_record.get(&record).is_some() {
            continue;
        }
        seen_record.insert(record);
        for unknown in unknows.iter() {
            if let Some(updated_spring) = insert_broken(springs, *unknown, *record) {
                let records = (&records[1..records.len()]).to_vec();
                insert_all(
                    updated_spring.as_str(),
                    original_records,
                    records,
                    set,
                    seen,
                );
            };
        }
    }
    if set.is_empty() {
        1
    } else {
        set.len()
    }
}

fn all_unknwons(springs: &str) -> Vec<usize> {
    let mut result = vec![];
    for i in 0..springs.len() {
        if &springs[i..i + 1] == "?" {
            result.push(i);
        };
    }
    result
}

// for this input ##..## it returns 2
fn how_many_broken_sequences(springs: &str) -> usize {
    let mut result = 0;
    let spltted_spring: Vec<char> = springs.chars().collect();
    for (i, _) in spltted_spring.iter().enumerate() {
        if i == 0 {
            if spltted_spring[i] == '#' {
                result += 1;
            }
        } else if spltted_spring[i - 1] == '.' && spltted_spring[i] == '#' {
            result += 1;
        }
    }
    result
}

fn insert_broken(springs: &str, index: usize, amount: usize) -> Option<String> {
    let mut springs_char = springs.chars().collect::<Vec<char>>();
    if index != 0 && springs_char[index - 1] == '#' {
        return None;
    }
    for i in 0..amount {
        if index + i == springs_char.len() {
            return None;
        }
        if springs_char[index + i] == '.' {
            return None;
        }
        if index != 0 && springs_char[index - 1] == '?' {
            springs_char[index - 1] = '.';
        }
        springs_char[index + i] = '#';
    }
    if index + amount != springs_char.len() {
        if springs_char[index + amount] == '#' {
            return None;
        }
        springs_char[index + amount] = '.';
    }
    Some(springs_char.iter().collect::<String>())
}

fn parse_input(input: &str) -> Vec<[&str; 2]> {
    input
        .lines()
        .map(|x| {
            let mut splited = x.split(' ');
            [splited.next().unwrap(), splited.next().unwrap()]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_exapmel_part_1() {
        let input = "???.### 1,1,3";
        let result = how_many_arragments(input);
        assert_eq!(1, result);
    }
    #[test]
    fn second_example_part_1() {
        let input = ".??..??...###. 1,1,3";
        let result = how_many_arragments(input);
        assert_eq!(4, result);
    }
    #[test]
    fn third_example_part_1() {
        let input = "?###???????? 3,2,1";
        let result = how_many_arragments(input);
        assert_eq!(10, result);
    }
    #[test]
    fn third_example_part_4() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        let result = how_many_arragments(input);
        assert_eq!(1, result);
    }
    #[test]
    fn third_example_part_5() {
        let input = "????.#...#... 4,1,1";
        let result = how_many_arragments(input);
        assert_eq!(1, result);
    }
    #[test]
    fn part_1_test() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let result = how_many_arragments(input);
        assert_eq!(21, result);
    }
}
