use std::collections::HashSet;

mod input;

struct Star {
    initial_x: usize,
    initial_y: usize,
    current_x: usize,
    current_y: usize,
}

impl Star {
    fn new(y: usize, x: usize) -> Star {
        Star {
            initial_x: x,
            initial_y: y,
            current_x: x,
            current_y: y,
        }
    }
    fn to_arr(&self) -> [usize; 2] {
        [self.current_y, self.current_x]
    }
}

fn main() {
    let result = solve_part_1(input::INPUT);
    println!("{:?}", result);
    let result = solve_part_2(input::INPUT, 1_000_000);
    println!("{:?}", result);
}

fn solve_part_1(input: &str) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let expanded_map = expand_input(map);
    let stars = find_all_stars(&expanded_map);
    get_sum(stars).try_into().unwrap()
}

fn solve_part_2(input: &str, multiplier: usize) -> i64 {
    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let empty_collums = find_empty_collums(&map);
    let empty_rows = find_empty_rows(&map);
    let mut stars = find_all_stars(&map);
    for row in empty_rows {
        for star in stars.iter_mut() {
            if star.initial_y > row {
                star.current_y += multiplier - 1;
            }
        }
    }
    for collum in empty_collums {
        for star in stars.iter_mut() {
            if star.initial_x > collum {
                star.current_x += multiplier - 1;
            }
        }
    }
    get_sum(stars)
}

fn get_sum(stars: Vec<Star>) -> i64 {
    let mut result = vec![];
    let mut galaxies_to_exclude = HashSet::new();
    for star in stars.iter() {
        result.push(find_distance_to_all_galaxies(
            star,
            &stars,
            &mut galaxies_to_exclude,
        ));
    }
    result.iter().flatten().sum()
}

fn find_all_stars(map: &Vec<Vec<char>>) -> Vec<Star> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, letter)| {
                    if letter == &'#' {
                        return Some(Star::new(y, x));
                    }
                    None
                })
                .collect::<Vec<Star>>()
        })
        .collect()
}

fn find_distance_to_all_galaxies(
    initial_location: &Star,
    galaxies: &Vec<Star>,
    galaxies_to_exclude: &mut HashSet<[[usize; 2]; 2]>,
) -> Vec<i64> {
    let mut result = vec![];
    for galaxy in galaxies {
        let vector = match sort_locatios(&initial_location.to_arr(), &galaxy.to_arr()) {
            Some(vector) => vector,
            None => {
                continue;
            }
        };
        if galaxies_to_exclude.get(&vector).is_none() {
            result.push(
                (initial_location.current_y as i64 - galaxy.current_y as i64).abs()
                    + (initial_location.current_x as i64 - galaxy.current_x as i64).abs(),
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
    result
}

fn expand_input(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let empty_rows = find_empty_rows(&map);
    let empty_collums = find_empty_collums(&map);
    for (i, row) in empty_rows.iter().enumerate() {
        map.insert(row + i, vec!['.'; map[0].len()]);
    }
    for (i, collum) in empty_collums.iter().enumerate() {
        map.iter_mut().for_each(|row| row.insert(collum + i, '.'));
    }
    map
}

fn find_empty_rows(map: &Vec<Vec<char>>) -> Vec<usize> {
    map.iter()
        .enumerate()
        .filter_map(|(y, row)| {
            if row.iter().all(|cell| cell != &'#') {
                return Some(y);
            }
            None
        })
        .collect()
}

fn find_empty_collums(map: &Vec<Vec<char>>) -> Vec<usize> {
    (0..map[0].len())
        .filter(|x| {
            let collum: Vec<char> = map.iter().map(|row| row[*x]).collect();
            collum.iter().all(|cell| cell != &'#')
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn shortest_paths() {
        let input = input::TEST_INPUT_EXPANDED;
        let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let stars = find_all_stars(&map);

        let result = get_sum(stars);
        assert_eq!(374, result);
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
    fn part_2_10() {
        let input = input::TEST_INPUT;
        let result = solve_part_2(input, 10);
        assert_eq!(result, 1030);
    }
    #[test]
    fn part_2_100() {
        let input = input::TEST_INPUT;
        let result = solve_part_2(input, 100);
        assert_eq!(result, 8410);
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
