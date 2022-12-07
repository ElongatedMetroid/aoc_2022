use std::{fs::File, io::Read, str::FromStr, cmp::Ordering};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum RoundMove {
    Rock,
    Paper,
    Scissors,
}

impl Ord for RoundMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            RoundMove::Rock => match other {
                RoundMove::Rock => Ordering::Equal,
                RoundMove::Paper => Ordering::Less,
                RoundMove::Scissors => Ordering::Greater,
            },
            RoundMove::Paper => match other {
                RoundMove::Rock => Ordering::Greater,
                RoundMove::Paper => Ordering::Equal,
                RoundMove::Scissors => Ordering::Less,
            },
            RoundMove::Scissors => match other {
                RoundMove::Rock => Ordering::Less,
                RoundMove::Paper => Ordering::Greater,
                RoundMove::Scissors => Ordering::Equal,
            },
        }
    }
}

impl FromStr for RoundMove {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("Invalid move str"),
        }
    }
}

struct Battle {
    player_score: usize,
    opponent_score: usize,
}

impl Battle {
    pub fn new() -> Self {
        Self {
            player_score: 0,
            opponent_score: 0,
        }
    }

    pub fn battle(&mut self, your_move: &RoundMove, opponent_move: &RoundMove) {
        match your_move {
            RoundMove::Rock => self.player_score += 1,
            RoundMove::Paper => self.player_score += 2,
            RoundMove::Scissors => self.player_score += 3,
        }

        match opponent_move {
            RoundMove::Rock => self.opponent_score += 1,
            RoundMove::Paper => self.opponent_score += 2,
            RoundMove::Scissors => self.opponent_score += 3,
        }

        match your_move.cmp(&opponent_move) {
            // Lost round
            Ordering::Less => self.opponent_score += 6,
            // Draw
            Ordering::Equal => {
                self.player_score += 3;
                self.opponent_score += 3;
            }
            // Won round
            Ordering::Greater => self.player_score += 6,
        }
    }

    pub fn battle_with_end(&mut self, opponent_move: &RoundMove, your_card: &Ordering) {
        let player_card = match opponent_move {
            RoundMove::Rock => match your_card {
                Ordering::Less => RoundMove::Scissors,
                Ordering::Equal => RoundMove::Rock,
                Ordering::Greater => RoundMove::Paper,
            },
            RoundMove::Paper => match your_card {
                Ordering::Less => RoundMove::Rock,
                Ordering::Equal => RoundMove::Paper,
                Ordering::Greater => RoundMove::Scissors,
            },
            RoundMove::Scissors => match your_card {
                Ordering::Less => RoundMove::Paper,
                Ordering::Equal => RoundMove::Scissors,
                Ordering::Greater => RoundMove::Rock,
            },
        };

        self.battle(&player_card, &opponent_move);
    }

    pub fn player_score(&self) -> usize {
        self.player_score
    }

    pub fn opponent_score(&self) -> usize {
        self.opponent_score
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
    let mut battle = Battle::new();
    input.trim_end().lines().for_each(|round| {
        let mut round_moves = round.split_whitespace();

        let opponent_move = round_moves.next().unwrap().parse::<RoundMove>().unwrap();
        let your_move = round_moves.next().unwrap().parse::<RoundMove>().unwrap();

        battle.battle(&your_move, &opponent_move);
    });

    println!("Score: {}-{}", battle.opponent_score(), battle.player_score());
}

fn part_two(input: &str) {
    let mut battle = Battle::new();
    input.trim_end().lines().for_each(|round| {
        let mut round_moves = round.split_whitespace();

        let opponent_move = round_moves.next().unwrap().parse::<RoundMove>().unwrap();
        let outcome = match round_moves.next().unwrap() {
            "X" => Ordering::Less,
            "Y" => Ordering::Equal,
            "Z" => Ordering::Greater,
            _ => {
                eprintln!("Invalid move outcome");
                std::process::exit(1);
            },
        };

        battle.battle_with_end(&opponent_move, &outcome);
    });

    println!("Score: {}-{}", battle.opponent_score(), battle.player_score());
}