use std::collections::HashSet;

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
enum Direction {
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

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    let result: String = grid
        .iter()
        .map(|row| row.iter().collect::<String>() + "\n")
        .collect();
    println!("{}", result);
}

fn generate_energized_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let row = vec!['.'; grid[0].len()];
    let mut energized_grid = vec![row; grid.len()];
    energized_grid[0][0] = '#';
    let mut beams = vec![Beam::new([0, 0], Direction::Right)];
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

pub fn solve(input: &str) -> usize {
    let grid = parse(input);
    let energized_grid = generate_energized_grid(grid);
    energized_grid
        .into_iter()
        .flatten()
        .filter(|square| square == &'#')
        .collect::<Vec<char>>()
        .len()
}
