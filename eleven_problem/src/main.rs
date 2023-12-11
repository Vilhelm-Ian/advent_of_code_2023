use std::collections::HashSet;

mod input;

fn main() {
    let result = solve_part_1(input::INPUT);
    println!("{:?}", result);
}

fn solve_part_1(input: &str) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let expanded_map = expand_input(map);
    get_sum(expanded_map)
}

fn solve_part_2(input: &str) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let expanded_map = expand_input(map);
    get_sum(expanded_map)
}

fn get_sum(map: Vec<Vec<char>>) -> i32 {
    let stars = find_all_stars(&map);
    let mut result = vec![];
    let mut galaxies_to_exclude = HashSet::new();
    for star in stars.iter() {
        result.push(find_distance_to_all_galaxies(
            star,
            &stars,
            &mut galaxies_to_exclude,
        ));

        // galaxies_to_exclude.insert(star);
    }
    result.iter().flatten().sum()
}

fn find_all_stars(map: &Vec<Vec<char>>) -> Vec<[usize; 2]> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, letter)| {
                    if letter == &'#' {
                        return Some([y, x]);
                    }
                    None
                })
                .collect::<Vec<[usize; 2]>>()
        })
        .collect()
}

fn find_distance_to_all_galaxies(
    initial_location: &[usize; 2],
    galaxies: &Vec<[usize; 2]>,
    galaxies_to_exclude: &mut HashSet<[[usize; 2]; 2]>,
) -> Vec<i32> {
    let mut result = vec![];
    for galaxy in galaxies {
        let vector = match sort_locatios(initial_location, galaxy) {
            Some(vector) => vector,
            None => continue,
        };
        if galaxies_to_exclude.get(&vector).is_none() {
            result.push(
                (initial_location[0] as i32 - galaxy[0] as i32).abs()
                    + (initial_location[1] as i32 - galaxy[1] as i32).abs(),
            );
            galaxies_to_exclude.insert(vector);
        }
    }
    result
}

fn sort_locatios(a: &[usize; 2], b: &[usize; 2]) -> Option<[[usize; 2]; 2]> {
    if a[0] > b[0] {
        return Some([*a, *b]);
    }
    if b[0] > a[0] {
        return Some([*b, *a]);
    }
    if a[1] > b[1] {
        return Some([*a, *b]);
    }
    if b[1] > a[1] {
        return Some([*b, *a]);
    }
    None
}

fn generate_new_locations(location: &[usize; 2], height: usize, width: usize) -> Vec<[usize; 2]> {
    let mut result = vec![];
    if location[0] + 1 < height {
        result.push([location[0] + 1, location[1]])
    }
    if location[1] + 1 < width {
        result.push([location[0], location[1] + 1])
    }
    if location[0] > 0 {
        result.push([location[0] - 1, location[1]])
    }
    if location[1] > 0 {
        result.push([location[0], location[1] - 1])
    }
    // println!("{:?}", result);
    // println!("{:?} {:?}", height, width);
    result
}

fn expand_input(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let empty_rows: Vec<usize> = map
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            if row.iter().all(|cell| cell != &'#') {
                return Some(y);
            }
            None
        })
        .collect();
    let empty_collums: Vec<usize> = (0..map[0].len())
        .filter(|x| {
            let collum: Vec<char> = map.iter().map(|row| row[*x]).collect();
            collum.iter().all(|cell| cell != &'#')
        })
        .collect();
    for (i, row) in empty_rows.iter().enumerate() {
        map.insert(row + i, vec!['.'; map[0].len()]);
    }
    for (i, collum) in empty_collums.iter().enumerate() {
        map.iter_mut().for_each(|row| row.insert(collum + i, '.'));
    }
    map
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn shortest_paths() {
        let input = input::TEST_INPUT_EXPANDED;
        let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let result = get_sum(map);
        assert_eq!(374, result);
    }
    #[test]
    fn shortest_paths_part_2() {
        let input = input::TEST_INPUT_EXPANDED;
        let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let result = get_sum(map);
        assert_eq!(8410, result);
    }
    #[test]
    fn expand() {
        let input = input::TEST_INPUT;
        let parsed: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let result = expand_input(parsed);
        let input_expanded = input::TEST_INPUT_EXPANDED;
        let parsed_expanded: Vec<Vec<char>> = input_expanded
            .lines()
            .map(|x| x.chars().collect())
            .collect();
        assert_eq!(result, parsed_expanded);
    }
    #[test]
    fn expand_just_collums() {
        let input = "#..\n#..\n#..";
        let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let result = expand_input(map);
        let expected = "#....\n#....\n#....\n";
        let expected: Vec<Vec<char>> = expected.lines().map(|x| x.chars().collect()).collect();
        assert_eq!(result, expected);
    }
    #[test]
    fn expand_just_rows() {
        let input = "###\n...\n...";
        let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let result = expand_input(map);
        let expected = "###\n...\n...\n...\n...";
        let expected: Vec<Vec<char>> = expected.lines().map(|x| x.chars().collect()).collect();
        assert_eq!(result, expected);
    }
    #[test]
    fn part_1() {
        let input = input::TEST_INPUT;
        let result = solve_part_1(input);
        assert_eq!(result, 374);
    }
    #[test]
    fn part_1_single_row_1() {
        let input = "#.#";
        let result = solve_part_1(input);
        assert_eq!(result, 3);
    }
    #[test]
    fn part_1_single_row_2() {
        let input = "###";
        let result = solve_part_1(input);
        assert_eq!(result, 4);
    }
    #[test]
    fn sort_test() {
        let a = [0, 1];
        let b = [3, 0];
        let result = sort_locatios(&a, &b);
        assert_eq!(result.unwrap(), [b, a]);
    }
    #[test]
    fn sort_test_1() {
        let b = [0, 1];
        let a = [3, 0];
        let result = sort_locatios(&a, &b);
        assert_eq!(result.unwrap(), [a, b]);
    }
    #[test]
    fn sort_test_2() {
        let a = [3, 1];
        let b = [3, 0];
        let result = sort_locatios(&a, &b);
        assert_eq!(result.unwrap(), [a, b]);
    }
    #[test]
    fn sort_test_4() {
        let b = [3, 1];
        let a = [3, 0];
        let result = sort_locatios(&a, &b);
        let result_2 = sort_locatios(&b, &a);
        assert_eq!(result.unwrap(), result_2.unwrap());
    }
}
