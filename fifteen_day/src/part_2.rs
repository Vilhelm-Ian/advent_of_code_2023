use crate::part_1;
use std::collections::{HashMap, HashSet};
//&a[a.len()-1..a.len()]

// fn parse_step(step: &str) -> i32 {
//     let label = step.split('=').collect().unwr;
//     return (part_1::hash_algorithm(l&step[0..step.len() - 1abel), focal_length);

// &step[0..step.len() - 1// }

fn add_to_box<'a>(
    label: &'a str,
    focal_length: &'a str,
    boxes: &mut Vec<Vec<[&'a str; 2]>>,
    indexes: &mut Vec<HashMap<&'a str, i32>>,
) {
    let box_ = part_1::hash_algorithm(label) as usize;
    if let Some(index) = indexes[box_].get(label) {
        boxes[box_][*index as usize] = [label, focal_length];
    } else {
        boxes[box_].push([label, focal_length]);
        indexes[box_].insert(label, boxes[box_].len() as i32 - 1);
    }
}

fn remove_from_box1(
    label: &str,
    boxes: &mut Vec<Vec<[&str; 2]>>,
    indexes: &mut Vec<HashMap<&str, i32>>,
) {
    let box_ = part_1::hash_algorithm(label) as usize;
    // boxes[box_index].push([label, focal_length]);
    if let Some(index) = indexes[box_].get(label) {
        let index = *index;
        boxes[box_].remove(index as usize);
        for (_key, value) in indexes[box_].iter_mut() {
            if *value > index {
                *value -= 1
            }
        }
        indexes[box_].remove(label);
    };
}

pub fn generate_book(sequence: &str) -> Vec<Vec<[&str; 2]>> {
    let parsed: Vec<&str> = sequence.split(',').collect();
    let mut boxes = vec![vec![]; 265];
    let mut indexes = vec![HashMap::new(); 265];
    for step in parsed {
        if &step[step.len() - 1..step.len()] == "-" {
            let label = &step[0..step.len() - 1];
            remove_from_box1(label, &mut boxes, &mut indexes);
        } else {
            let splitted = step.split('=').collect::<Vec<&str>>();
            let [label, focal_length] = [splitted[0], splitted[1]];
            add_to_box(label, focal_length, &mut boxes, &mut indexes);
        }
    }
    boxes
}

pub fn solve(sequence: &str) -> usize {
    let mut sum = 0;
    let book = generate_book(sequence);
    for (y, box_) in book.iter().enumerate() {
        for (x, [_, focal_length]) in box_.iter().enumerate() {
            sum += (y + 1) * (x + 1) * focal_length.parse::<usize>().unwrap();
        }
    }
    sum
}
