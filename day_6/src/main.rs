use std::{fs::File, io::Read, collections::HashSet};

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
    let binding = (1..).zip(input.chars()).collect::<Vec<(usize, char)>>();
    let data_windows = binding.windows(4);

    for window in data_windows {
        let mut start_of_packet_window = String::new();

        for (_, ch) in window {
            start_of_packet_window.push(*ch);
        }

        // Start of packet marker found
        if !repeated_chars(&start_of_packet_window) {
            println!("first start of message marker at {}", window.last().unwrap().0);
            return;
        }
    }
}

fn part_two(input: &str) {
    let binding = (1..).zip(input.chars()).collect::<Vec<(usize, char)>>();
    let data_windows = binding.windows(14);

    for window in data_windows {
        let mut start_of_packet_window = String::new();

        for (_, ch) in window {
            start_of_packet_window.push(*ch);
        }

        // Start of packet marker found
        if !repeated_chars(&start_of_packet_window) {
            println!("first start of packet marker at {}", window.last().unwrap().0);
            return;
        }
    }
}

fn repeated_chars(s: &str) -> bool {
    let mut seen_chars = HashSet::new();

    for check_ch in s.chars() {
        if seen_chars.contains(&check_ch) {
            return true;
        } else {
            seen_chars.insert(check_ch);
        }
    }

    false
}

#[test]
fn repeated_chars_works() {
    assert!(repeated_chars("wtqw"));
    assert!(!repeated_chars("qwer"));
}
