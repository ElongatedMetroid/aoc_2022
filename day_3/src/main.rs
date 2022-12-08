use std::{fs::File, io::Read};

use itertools::Itertools;

trait GetPriority {
    fn get_priority(&self) -> isize;
}

impl GetPriority for char {
    fn get_priority(&self) -> isize {
        if self.is_lowercase() {
            *self as isize - 96
        } else {
            *self as isize - 38
        }
    }
}

fn main() {
    let input = {
        let mut f = File::open("input").unwrap();
        let mut input = String::new();
        f.read_to_string(&mut input).unwrap();
        input
    };

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut priority_sum = 0;
    input.lines().for_each(|rucksack| {
        let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

        priority_sum += common_char(vec![first_compartment, second_compartment])
            .unwrap()
            .get_priority();
    });

    println!("{}", priority_sum);
}

fn part_two(input: &str) {
    let mut priority_sum = 0;
    for group in &input.lines().into_iter().chunks(3) {
        priority_sum += common_char(group.collect_vec()).unwrap().get_priority();
    }

    println!("{}", priority_sum);
}

fn common_char(words: Vec<&str>) -> Option<char> {
    for word in &words {
        for c in word.chars() {
            if words.iter().all(|w| w.contains(c)) {
                return Some(c);
            }
        }
    }
    None
}
