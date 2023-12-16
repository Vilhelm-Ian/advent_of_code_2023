use indicatif::{MultiProgress, ProgressBar};
use std::collections::HashSet;
use std::thread;

#[derive(Clone, PartialEq, Eq)]
struct Beam {
    direction: Direction,
    location: Location,
}

impl Beam {
    fn new(location: [usize; 2], direction: Direction) -> Beam {
        Beam {
            direction,
            location: Location {
                y: location[0],
                x: location[1],
            },
        }
    }
    fn update_location(&mut self, grid: &Vec<Vec<char>>) -> Option<Location> {
        match self.direction {
            Direction::Left => {
                if self.location.x > 0 {
                    self.location.x -= 1;
                    Some(self.location.clone())
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.location.x < grid[0].len() - 1 {
                    self.location.x += 1;
                    Some(self.location.clone())
                } else {
                    None
                }
            }
            Direction::Up => {
                if self.location.y > 0 {
                    self.location.y -= 1;
                    Some(self.location.clone())
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.location.y < grid.len() - 1 {
                    self.location.y += 1;
                    Some(self.location.clone())
                } else {
                    None
                }
            }
        }
    }
    fn update_direction(&mut self, grid: &Vec<Vec<char>>) -> Option<Beam> {
        match (grid[self.location.y][self.location.x], &self.direction) {
            ('-', Direction::Up | Direction::Down) => {
                self.direction = Direction::Right;
                let mut new_beam = self.clone();
                new_beam.direction = Direction::Left;
                Some(new_beam)
            }

            ('-', _) => None,
            ('|', Direction::Left | Direction::Right) => {
                self.direction = Direction::Up;
                let mut new_beam = self.clone();
                new_beam.direction = Direction::Down;
                Some(new_beam)
            }
            ('|', _) => None,
            ('/', _) => {
                self.direction = self.reflect_rigtward();
                None
            }
            ('\\', _) => {
                self.direction = self.reflect_leftward();
                None
            }
            ('.', _) => None,
            (_, _) => {
                println!("{:?}", grid[self.location.y][self.location.x]);
                panic!("invalid square")
            }
        }
    }
    fn reflect_rigtward(&self) -> Direction {
        match self.direction {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
        }
    }
    fn reflect_leftward(&self) -> Direction {
        match self.direction {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}
#[derive(Clone, PartialEq, Eq)]
struct Location {
    x: usize,
    y: usize,
}

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    let result: String = grid
        .iter()
        .map(|row| row.iter().collect::<String>() + "\n")
        .collect();
    println!("{}", result);
}

pub fn find_horizontal_ranges(grid: &Vec<Vec<char>>) -> Vec<[[usize; 2]; 2]> {
    let mut ranges = vec![];
    for (y, row) in grid.iter().enumerate() {
        let mut range_start = [y, 0];
        for (x, square) in row.iter().enumerate() {
            if (square == &'|' || square == &'\\' || square == &'/' || x == row.len() - 1) && x != 0
            {
                let range_end = [y, x];
                ranges.push([range_start, range_end]);
                range_start = range_end;
            }
        }
    }
    ranges
}

pub fn find_vertical_ranges(grid: &Vec<Vec<char>>) -> Vec<[[usize; 2]; 2]> {
    let mut ranges = vec![];
    for x in 0..grid[0].len() {
        let mut range_start = [0, x];
        for (y, row) in grid.iter().enumerate() {
            let square = row[x];
            if (square == '-' || square == '\\' || square == '/' || y == grid.len() - 1) && y != 0 {
                let range_end = [y, x];
                ranges.push([range_start, range_end]);
                range_start = range_end;
            }
        }
    }
    ranges
}

fn generate_energized_grid(
    grid: &Vec<Vec<char>>,
    starting_location: [usize; 2],
    starting_direction: Direction,
) -> Vec<Vec<char>> {
    let row = vec!['.'; grid[0].len()];
    let mut energized_grid = vec![row; grid.len()];
    energized_grid[starting_location[0]][starting_location[1]] = '#';
    let mut beams = vec![Beam::new(starting_location, starting_direction)];
    let mut new_beams = vec![];
    let mut locations_hashed = HashSet::new();
    let mut locations = get_locations(&beams);
    let mut cycle_detected = false;
    while !beams.is_empty() && !cycle_detected {
        let mut indexes_to_remove = vec![];
        for (i, beam) in beams.iter_mut().enumerate() {
            match beam.update_location(&grid) {
                Some(location) => {
                    energized_grid[location.y][location.x] = '#';
                }
                None => {
                    indexes_to_remove.push(i);
                }
            };
            if let Some(new_beam) = beam.update_direction(&grid) {
                if !new_beams.contains(&new_beam) {
                    new_beams.push(new_beam);
                }
            }
        }
        beams.append(&mut new_beams);
        indexes_to_remove.iter().enumerate().for_each(|(i, index)| {
            beams.remove(index - i);
        });
        locations_hashed.insert(locations.clone());
        locations = get_locations(&beams);

        cycle_detected = is_cycle_detected(&locations_hashed, &locations);
    }
    energized_grid
}

fn is_cycle_detected(set: &HashSet<Vec<[usize; 2]>>, beam_locations: &Vec<[usize; 2]>) -> bool {
    set.get(beam_locations).is_some()
}

fn get_locations(beams: &Vec<Beam>) -> Vec<[usize; 2]> {
    beams
        .iter()
        .map(|beam| [beam.location.y, beam.location.x])
        .collect()
}

fn is_edge_location(location: [usize; 2], grid: &Vec<Vec<char>>) -> bool {
    if location[0] == 0 || location[0] == grid.len() - 1 {
        return true;
    }
    if location[1] == 0 || location[1] == grid[0].len() - 1 {
        return true;
    }
    false
}

pub fn solve(input: &str) -> usize {
    let grid = parse(input);
    let mut horizontal_ranges = find_horizontal_ranges(&grid);
    let mut vertical_ranges = find_vertical_ranges(&grid);
    horizontal_ranges.retain(|range| is_edge_location(range[0], &grid));
    vertical_ranges.retain(|range| is_edge_location(range[0], &grid));
    let horizontal_length = horizontal_ranges.len();
    let vertical_length = vertical_ranges.len();
    let grid_1 = grid.clone();
    let grid_2 = grid.clone();
    let multi = MultiProgress::new();
    let bar_1 = multi.add(ProgressBar::new(horizontal_length as u64));
    let bar_2 = multi.add(ProgressBar::new(vertical_length as u64));
    let horizontal_thread = thread::spawn(move || {
        let mut lengths = vec![];
        for [start_range, end_range] in horizontal_ranges {
            bar_1.inc(1);
            let grid_1_1 = grid_1.clone();
            let thread_1 = thread::spawn(move || {
                let energized_grid = generate_energized_grid(&grid_1_1, end_range, Direction::Left);
                get_lava_length(&energized_grid)
            });
            let grid_1_2 = grid_1.clone();
            let thread_2 = thread::spawn(move || {
                let energized_grid =
                    generate_energized_grid(&grid_1_2, start_range, Direction::Right);
                get_lava_length(&energized_grid)
            });
            let lava_length = thread_1.join().unwrap();
            lengths.push(lava_length);
            let lava_length = thread_2.join().unwrap();
            lengths.push(lava_length);
        }
        bar_1.finish();
        *lengths.iter().max().unwrap()
    });
    let vertical_thread = thread::spawn(move || {
        let mut lengths = vec![];
        for [start_range, end_range] in vertical_ranges {
            bar_2.inc(1);
            let grid_2_1 = grid_2.clone();
            let thread_1 = thread::spawn(move || {
                let energized_grid = generate_energized_grid(&grid_2_1, end_range, Direction::Down);
                get_lava_length(&energized_grid)
            });
            let grid_2_2 = grid_2.clone();
            let thread_2 = thread::spawn(move || {
                let energized_grid = generate_energized_grid(&grid_2_2, start_range, Direction::Up);
                get_lava_length(&energized_grid)
            });
            let lava_length = thread_1.join().unwrap();
            lengths.push(lava_length);
            let lava_length = thread_2.join().unwrap();
            lengths.push(lava_length);
        }
        bar_2.finish();
        *lengths.iter().max().unwrap()
    });
    let horizontal_max = horizontal_thread.join().unwrap();
    let vertical_max = vertical_thread.join().unwrap();
    *[horizontal_max, vertical_max].iter().max().unwrap()
}

fn get_lava_length(grid: &Vec<Vec<char>>) -> usize {
    grid.into_iter()
        .flatten()
        .filter(|square| square == &&'#')
        .collect::<Vec<&char>>()
        .len()
}
