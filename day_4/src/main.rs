use std::{fs::File, io::Read, ops::RangeInclusive};

trait ContainsRange<T> {
    fn contains_range(&self, range: &RangeInclusive<T>) -> bool;
}

impl<T> ContainsRange<T> for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, range: &RangeInclusive<T>) -> bool {
        self.start() >= range.start() && self.end() <= range.end()
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
    let mut range_contains_other = 0;
    input.lines().for_each(|pair| {
        let ranges: Vec<RangeInclusive<usize>> = pair
            .split(',')
            .map(|elf| {
                let mut nums = elf.split('-');

                RangeInclusive::new(
                    nums.next().unwrap().parse::<usize>().unwrap(),
                    nums.next().unwrap().parse::<usize>().unwrap(),
                )
            })
            .collect();

        let range1 = &ranges[0];
        let range2 = &ranges[1];

        if range1.contains_range(range2) || range2.contains_range(range1) {
            range_contains_other += 1;
        }
    });

    println!(
        "Amount of pairs in which one range contains another: {}",
        range_contains_other
    );
}

fn part_two(input: &str) {
    let mut range_overlaps = 0;
    input.lines().for_each(|pair| {
        let mut ranges: Vec<Option<RangeInclusive<usize>>> = pair
            .split(',')
            .map(|elf| {
                let mut nums = elf.split('-');

                Some(RangeInclusive::new(
                    nums.next().unwrap().parse::<usize>().unwrap(),
                    nums.next().unwrap().parse::<usize>().unwrap(),
                ))
            })
            .collect();

        let mut range1 = ranges[0].take().unwrap();
        let range2 = ranges[1].take().unwrap();

        if range1.any(|n| range2.contains(&n)) {
            range_overlaps += 1;
        }
    });

    println!(
        "Amount of pairs in which one range overlaps another: {}",
        range_overlaps
    );
}
