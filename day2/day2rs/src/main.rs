use std::{error::Error, str::FromStr};

fn main() {
    a();
    b();
}

fn a() {
    let points =
        include_str!("./input.txt")
            .lines()
            .into_iter()
            .fold(0, |mut acc: i32, item: &str| {
                let game = item.split(" ").collect::<Vec<&str>>();
                acc += determine_outcome(
                    RockPaperScissors::from_str(game[0]).unwrap(),
                    RockPaperScissors::from_str(game[1]).unwrap(),
                );
                acc
            });
    println!("{} points", points);
}

fn b() {
    let points =
        include_str!("./input.txt")
            .lines()
            .into_iter()
            .fold(0, |mut acc: i32, item: &str| {
                let game = item.split(" ").collect::<Vec<&str>>();
                let them = RockPaperScissors::from_str(game[0]).unwrap();
                let outcome = match game[1] {
                    "X" => Outcome::Lose,
                    "Y" => Outcome::Draw,
                    "Z" => Outcome::Win,
                    v => panic!("what's {}?", v),
                };
                let us = choose_play(outcome, them);
                acc += determine_outcome(RockPaperScissors::from_str(game[0]).unwrap(), us);
                acc
            });
    println!("{} points", points);
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

#[derive(Clone, Copy, Debug)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl PartialEq for RockPaperScissors {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RockPaperScissors::Rock, RockPaperScissors::Rock) => true,
            (RockPaperScissors::Paper, RockPaperScissors::Paper) => true,
            (RockPaperScissors::Scissors, RockPaperScissors::Scissors) => true,
            (_a, _b) => false,
        }
    }
}

impl PartialOrd for RockPaperScissors {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }

        match (self, other) {
            (RockPaperScissors::Rock, RockPaperScissors::Paper) => Some(std::cmp::Ordering::Less),
            (RockPaperScissors::Rock, RockPaperScissors::Scissors) => {
                Some(std::cmp::Ordering::Greater)
            }
            (RockPaperScissors::Paper, RockPaperScissors::Rock) => {
                Some(std::cmp::Ordering::Greater)
            }
            (RockPaperScissors::Paper, RockPaperScissors::Scissors) => {
                Some(std::cmp::Ordering::Less)
            }
            (RockPaperScissors::Scissors, RockPaperScissors::Rock) => {
                Some(std::cmp::Ordering::Less)
            }
            (RockPaperScissors::Scissors, RockPaperScissors::Paper) => {
                Some(std::cmp::Ordering::Greater)
            }
            (_, _) => unreachable!(),
        }
    }
}

impl RockPaperScissors {
    fn points(&self) -> i32 {
        match self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
        }
    }

    fn outcome(us: Self, them: Self) -> i32 {
        if us > them {
            6
        } else if us == them {
            3
        } else {
            0
        }
    }
}

impl FromStr for RockPaperScissors {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            v => panic!("{} invalid", v),
        }
    }
}

fn determine_outcome(them: RockPaperScissors, us: RockPaperScissors) -> i32 {
    RockPaperScissors::outcome(us, them) + us.points()
}

fn choose_play(outcome: Outcome, them: RockPaperScissors) -> RockPaperScissors {
    match (outcome, them) {
        (Outcome::Lose, RockPaperScissors::Rock) => RockPaperScissors::Scissors,
        (Outcome::Lose, RockPaperScissors::Paper) => RockPaperScissors::Rock,
        (Outcome::Lose, RockPaperScissors::Scissors) => RockPaperScissors::Paper,
        (Outcome::Draw, v) => v,
        (Outcome::Win, RockPaperScissors::Rock) => RockPaperScissors::Paper,
        (Outcome::Win, RockPaperScissors::Paper) => RockPaperScissors::Scissors,
        (Outcome::Win, RockPaperScissors::Scissors) => RockPaperScissors::Rock,
    }
}
