use std::collections::HashMap;

mod input;

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
    let result = generate_loop(input::INPUT);
    println!("{:?}", result);
    // println!("Hello, world!");
}

fn generate_loop(input: &str) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let hash_map = generate_hash_map(&map);
    let starting_point = find_starting_position(&map);
    //there are two ways to go about this
    // one is to search the end of the loop of both ends witch would be twice as fast tot do this i need to change the function find next_node_location to return an vector of positions
    // the other is to search the entire loop until I reach the start and and divide by two
    // I am going to try with the first approach
    let (next_node_location, mut prev_direction) = next_node_and_direction(
        &map,
        &hash_map,
        hash_map.get(&starting_point).unwrap(),
        None,
    );
    let mut current_node = hash_map
        .get(&next_node_location)
        .expect("node dosen't exist in hashmap");
    let mut i = 1;
    while current_node.position != starting_point {
        println!(
            "current node is {:?} current direction is {:?}",
            current_node.pipe_type, prev_direction
        );
        let (next_node_location, direction) =
            next_node_and_direction(&map, &hash_map, current_node, Some(&prev_direction));
        prev_direction = direction;
        current_node = hash_map
            .get(&next_node_location)
            .expect("node dosen't exist in hashmap");
        println!(
            "current node is {:?} current direction is {:?}",
            current_node.pipe_type, prev_direction
        );
        println!("\n");
        i += 1;
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
            // println!("the directon is {:?}", direction);
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
    let y = current_position[0] - next_position[0];
    let x = current_position[1] - next_position[1];
    match (y, x) {
        (0, 1) => Some(Direction::Right),
        (0, -1) => Some(Direction::Left),
        (1, 0) => Some(Direction::Up),
        (-1, 0) => Some(Direction::Down),
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

fn next_node_and_direction(
    map: &Vec<Vec<char>>,
    pipes: &HashMap<[usize; 2], Pipe>,
    current_pipe: &Pipe,
    prev_direction: Option<&Direction>,
) -> ([usize; 2], Direction) {
    println!("{:?}", current_pipe);
    for y in 0..3 {
        for x in 0..3 {
            let current_position = current_pipe.position;
            let new_position = [
                (current_position[0] + y) as i32 - 1,
                (current_position[1] + x) as i32 - 1,
            ];
            // println!("new_position {:?}", new_position);
            let direction = match get_direction(
                [current_position[0] as i32, current_position[1] as i32],
                new_position,
            ) {
                Some(direction) => direction,
                None => continue,
            };
            // println!("new_position {:?} direction {:?}", new_position, direction);
            if let Some(prev_direction) = prev_direction {
                if is_opposite(prev_direction, &direction) {
                    continue;
                }
            }
            let next_pipe = match pipes.get(&[new_position[0] as usize, new_position[1] as usize]) {
                Some(pipe) => pipe,
                None => continue,
            };
            if !get_next_node_location(next_pipe, &direction) {
                continue;
            }
            return (
                [new_position[0] as usize, new_position[1] as usize],
                direction,
            );
        }
    }
    panic!("couldn't find next node");
}

// todo!("rename later")
fn get_next_node_location(pipe: &Pipe, direction: &Direction) -> bool {
    // println!("{:?} {:?}", pipe.pipe_type, direction);
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
    use std::collections::hash_map;

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
        let map = TEST_INPUT_1
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let hash_map: HashMap<[usize; 2], Pipe> = generate_hash_map(&map);
        let current_pipe = Pipe::new(1, 2, PipeType::Horizontal);
        let result =
            next_node_and_direction(&map, &hash_map, &current_pipe, Some(&Direction::Left));
        println!("print {:?} {:?}", result.0, result.1);
        assert!(matches!(result, ([1, 3], Direction::Left)));
    }
    #[test]
    fn first_test_part_1_part_1() {
        let result = get_direction([40, 30], [40, 29]).unwrap();
        assert!(matches!(result, Direction::Right));
    }
    #[test]
    fn second_test_part_1() {
        let result = generate_loop(TEST_INPUT_2);
        assert_eq!(16, result);
    }
    #[test]
    fn direction_test_1() {
        let result = get_direction([40, 30], [39, 30]).unwrap();
        assert!(matches!(result, Direction::Up));
    }
    #[test]
    fn direction_test_3() {
        let result = get_direction([40, 30], [41, 30]).unwrap();
        assert!(matches!(result, Direction::Down));
    }
    #[test]
    fn direction_test_2() {
        let result = get_direction([40, 30], [40, 31]).unwrap();
        assert!(matches!(result, Direction::Left));
    }
    #[test]
    fn direction_test_4() {
        let result = get_direction([40, 30], [40, 29]).unwrap();
        assert!(matches!(result, Direction::Right));
    }
    #[test]
    fn test_from_problem_input() {
        let input = "F77S7
|L-J|
L---J";
        let result = generate_loop(input);
        assert_eq!(input.len() as i32, result);
    }
}
