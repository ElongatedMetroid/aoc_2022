use std::{fs::File, io::Read};

fn main() {
    let input = {
        let mut f = File::open("input").unwrap();
        let mut input = String::new();
        f.read_to_string(&mut input).unwrap();
        input
    };

    let mut calories: Vec<usize> = input
        .trim_end()
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect();

    calories.sort();

    let top_three = &calories[calories.len() - 3..calories.len()];
    let mut top_three_iter = top_three.iter().rev();
    println!(
        "Amount of calories the elf carrying the most calories has: {}",
        top_three_iter.next().unwrap()
    );

    println!(
        "2.) {}\n3.) {}\nTotal: {}",
        top_three_iter.next().unwrap(),
        top_three_iter.next().unwrap(),
        top_three.iter().sum::<usize>()
    );
}
