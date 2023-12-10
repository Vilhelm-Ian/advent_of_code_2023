use std::collections::HashMap;
use std::io::Cursor;
use std::ops::Add;

#[derive(Debug)]
struct Pipe<'a> {
    distance_from_start: Option<i32>,
    next_node: Option<&'a Pipe<'a>>,
    pipe_type: PipeType,
    position: [usize; 2],
}

impl<'a> Pipe<'a> {
    fn new(y: usize, x: usize, pipe_type: PipeType) -> Pipe<'a> {
        Self {
            position: [y, x],
            pipe_type,
            next_node: None,
            distance_from_start: None,
        }
    }
}

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(y: usize, x: usize) -> Position {
        Position { x, y }
    }
}

#[derive(Debug)]
enum PipeType {
    Vertical,
    Horizontal,
    Ground,
    LBend,
    JBend,
    SevenBend,
    FBend,
    Starting,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    generate_loop("hello");
    println!("Hello, world!");
}

fn generate_loop(input: &str) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let hash_map = generate_hash_map(&map);
    let starting_point = find_starting_position(&map);
    //there are two ways to go about this
    // one is to search the end of the loop of both ends witch would be twice as fast tot do this i need to change the function find next_node_location to return an vector of positions
    // the other is to search the entire loop until I reach the start and and divide by two
    // I am going to try with the first approach
    let (next_node_location, mut prev_direction) = find_first_valid(starting_point, &hash_map);
    let mut prev_direction = Direction::Left;
    let mut current_node = hash_map
        .get(&next_node_location)
        .expect("node dosen't exist in hashmap");
    let mut i = 1;
    while current_node.position != starting_point {
        println!("\n");
        let (next_node_location, direction) =
            name_later(&map, &hash_map, current_node, &mut prev_direction);
        prev_direction = direction;
        current_node = hash_map
            .get(&next_node_location)
            .expect("node dosen't exist in hashmap");
        i += 1;
        if i == 18 {
            break;
        }
    }
    i
}

fn find_first_valid(
    starting_position: [usize; 2],
    map: &HashMap<[usize; 2], Pipe>,
) -> ([usize; 2], Direction) {
    for y in 0..3 {
        for x in 0..3 {
            if starting_position[0] + y == 0 || starting_position[1] + x == 0 {
                continue;
            }
            let next_position = [starting_position[0] + y - 1, starting_position[1] + x - 1];
            let starting_position = [starting_position[0] as i32, starting_position[1] as i32];
            let direction = match get_direction(
                starting_position,
                [next_position[0] as i32, next_position[1] as i32],
            ) {
                Some(direction) => direction,
                None => continue,
            };
            println!("the directon is {:?}", direction);
            let next_pipe = map.get(&next_position).expect("pipe dosen't exist");
            if get_next_node_location(next_pipe, &direction) {
                return (next_position, direction);
            }
        }
    }
    panic!("couldn't find valid pipe around starting location");
}

fn generate_hash_map(map: &Vec<Vec<char>>) -> HashMap<[usize; 2], Pipe> {
    let mut pipes = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            let pipe_type = char_to_pipe(pipe);
            pipes.insert([y, x], Pipe::new(y, x, pipe_type));
        }
    }
    pipes
}

fn find_starting_position(map: &Vec<Vec<char>>) -> [usize; 2] {
    for (y, row) in map.iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            if pipe == &'S' {
                return [y, x];
            }
        }
    }
    panic!("no starting position");
}

fn char_to_pipe(letter: &char) -> PipeType {
    match letter {
        '|' => PipeType::Vertical,
        '-' => PipeType::Horizontal,
        '.' => PipeType::Ground,
        'L' => PipeType::LBend,
        'J' => PipeType::JBend,
        '7' => PipeType::SevenBend,
        'F' => PipeType::FBend,
        'S' => PipeType::Starting,
        _ => panic!("invavild characther: {letter} can't create pipe"),
    }
}

fn get_direction(current_position: [i32; 2], next_position: [i32; 2]) -> Option<Direction> {
    let y = current_position[0] - next_position[0] + 1;
    let x = current_position[1] - next_position[1] + 1;
    match (y, x) {
        (0, 1) => Some(Direction::Up),
        (1, 0) => Some(Direction::Left),
        (1, 2) => Some(Direction::Right),
        (2, 1) => Some(Direction::Down),
        _ => None,
    }
}

fn is_opposite(prev_direction: &Direction, direction: &Direction) -> bool {
    match (prev_direction, direction) {
        (Direction::Left, Direction::Right) => true,
        (Direction::Right, Direction::Left) => true,
        (Direction::Down, Direction::Up) => true,
        (Direction::Up, Direction::Down) => true,
        _ => false,
    }
}

fn name_later(
    map: &Vec<Vec<char>>,
    pipes: &HashMap<[usize; 2], Pipe>,
    current_pipe: &Pipe,
    prev_direction: &Direction,
) -> ([usize; 2], Direction) {
    for y in 0..3 {
        for x in 0..3 {
            let direction = match (y, x) {
                (0, 1) => Direction::Up,
                (1, 0) => Direction::Right,
                (1, 2) => Direction::Left,
                (2, 1) => Direction::Down,
                _ => continue,
            };
            if (y == 0 && current_pipe.position[0] == 0)
                || (x == 0 && current_pipe.position[1] == 0)
            {
                continue;
            }
            if is_opposite(prev_direction, &direction) {
                continue;
            }
            let next_pipe = match pipes.get(&[
                current_pipe.position[0] + y - 1,
                current_pipe.position[1] + x - 1,
            ]) {
                Some(pipe) => pipe,
                None => continue,
            };
            println!(
                "current {:?} is {:?} next {:?} is {:?} ",
                current_pipe.position,
                current_pipe.pipe_type,
                next_pipe.position,
                next_pipe.pipe_type
            );

            // let new_position = [position[0] + y - 1, position[1] + x - 1];
            if get_next_node_location(next_pipe, &direction) {
                return (next_pipe.position, direction);
            }
        }
    }
    panic!("couldn't find next node");
}

// todo!("rename later")
fn get_next_node_location(pipe: &Pipe, direction: &Direction) -> bool {
    println!("{:?} {:?}", pipe.pipe_type, direction);
    match (direction, &pipe.pipe_type) {
        (Direction::Up, PipeType::SevenBend) => true,
        (Direction::Up, PipeType::FBend) => true,
        (Direction::Up, PipeType::Vertical) => true,
        (Direction::Down, PipeType::Vertical) => true,
        (Direction::Down, PipeType::LBend) => true,
        (Direction::Down, PipeType::JBend) => true,
        (Direction::Left, PipeType::SevenBend) => true,
        (Direction::Left, PipeType::Horizontal) => true,
        (Direction::Left, PipeType::JBend) => true,
        (Direction::Right, PipeType::FBend) => true,
        (Direction::Right, PipeType::Horizontal) => true,
        (Direction::Right, PipeType::LBend) => true,
        (_, PipeType::Starting) => true,
        (_, _) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const TEST_INPUT_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
    pub const TEST_INPUT_2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    #[test]
    fn first_test_part_1() {
        let result = generate_loop(TEST_INPUT_1);
        assert_eq!(8, result);
    }
    #[test]
    fn second_test_part_1() {
        let result = generate_loop(TEST_INPUT_2);
        assert_eq!(16, result);
    }
}
