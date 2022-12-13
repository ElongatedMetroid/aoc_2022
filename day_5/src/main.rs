#![feature(iter_array_chunks)]
use std::{collections::{VecDeque, BTreeMap}, fs::File, io::Read};

fn main() {
    let input = {
        let mut f = File::open("input.txt").unwrap();
        let mut input = String::new();
        f.read_to_string(&mut input).unwrap();
        input
    };

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    // Use the column as a key to access a vector of characters
    let mut crates: BTreeMap<usize, VecDeque<char>> = BTreeMap::new();
    let (starting_stacks, rearrangement_procedures) = input.split_once("\n\n").unwrap();
    let starting_stacks = starting_stacks.rsplit_once('\n').unwrap().0;

    starting_stacks.lines().for_each(|crate_row| {
        // Add a space onto the end of the crate_row string so we dont have to deal with remainders
        let crate_row = format!("{crate_row} ");
        let crate_rows = crate_row.chars().array_chunks::<4>();

        crate_rows
            .enumerate()
            .for_each(|(col, [_, crate_ch, _, _])| {
                if crate_ch != ' ' {
                    crates
                        .entry(col + 1)
                        .and_modify(|column| column.push_back(crate_ch))
                        .or_insert_with(|| vec![crate_ch].into());
                }
            });
    });

    rearrangement_procedures.lines().for_each(|procedure| {
        let mut words = procedure.split_whitespace();

        let amount = words.nth(1).unwrap().parse::<usize>().unwrap();
        let from = words.nth(1).unwrap().parse::<usize>().unwrap();
        let to = words.nth(1).unwrap().parse::<usize>().unwrap();

        let crates_from = crates
            .get_mut(&from)
            .map(|column| {
                let mut from_crates = Vec::new();

                for _ in 0..amount {
                    from_crates.push(column.pop_front().unwrap());
                }

                from_crates
            }).unwrap();

        crates.get_mut(&to).map(|column| {
            for crate_ch in crates_from {
                column.push_front(crate_ch);
            }
        });
    });

    for col in crates.values() {
        print!("{}", col[0]);
    }
    println!();
}

fn part_two(input: &str) {
        // Use the column as a key to access a vector of characters
        let mut crates: BTreeMap<usize, VecDeque<char>> = BTreeMap::new();
        let (starting_stacks, rearrangement_procedures) = input.split_once("\n\n").unwrap();
        let starting_stacks = starting_stacks.rsplit_once('\n').unwrap().0;
    
        starting_stacks.lines().for_each(|crate_row| {
            // Add a space onto the end of the crate_row string so we dont have to deal with remainders
            let crate_row = format!("{crate_row} ");
            let crate_rows = crate_row.chars().array_chunks::<4>();
    
            crate_rows
                .enumerate()
                .for_each(|(col, [_, crate_ch, _, _])| {
                    if crate_ch != ' ' {
                        crates
                            .entry(col + 1)
                            .and_modify(|column| column.push_back(crate_ch))
                            .or_insert_with(|| vec![crate_ch].into());
                    }
                });
        });
    
        rearrangement_procedures.lines().for_each(|procedure| {
            let mut words = procedure.split_whitespace();
    
            let amount = words.nth(1).unwrap().parse::<usize>().unwrap();
            let from = words.nth(1).unwrap().parse::<usize>().unwrap();
            let to = words.nth(1).unwrap().parse::<usize>().unwrap();
    
            let mut crates_from = crates
                .get_mut(&from)
                .map(|column| {
                    let mut from_crates = Vec::new();
    
                    for _ in 0..amount {
                        from_crates.push(column.pop_front().unwrap());
                    }
    
                    from_crates
                }).unwrap();

            crates_from.reverse();
    
            crates.get_mut(&to).map(|column| {
                for crate_ch in crates_from {
                    column.push_front(crate_ch);
                }
            });
        });
    
        for col in crates.values() {
            print!("{}", col[0]);
        }
        println!();
}