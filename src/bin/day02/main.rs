enum RockPaperScissorsResult {
    Loss,
    Draw,
    Win
}

impl TryFrom<char> for RockPaperScissorsResult {
    type Error = RockPaperScissorsError;

    fn try_from(c : char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(RockPaperScissorsResult::Loss),
            'Y' => Ok(RockPaperScissorsResult::Draw),
            'Z' => Ok(RockPaperScissorsResult::Win),
            _ => Err(RockPaperScissorsError::CharParsing),
        }
    }
}

struct RockPaperScissorsRoundGuide {
    own : RockPaperScissorsResult,
    other : RockPaperScissors,
}

impl RockPaperScissorsRoundGuide {
    fn play(&self) -> RockPaperScissors{
        match (&self.own, &self.other) {
            (RockPaperScissorsResult::Loss, RockPaperScissors:: Rock) => {
                RockPaperScissors::Scissors
            }
            (RockPaperScissorsResult::Draw, RockPaperScissors::Rock) => {
                RockPaperScissors::Rock
            }
            (RockPaperScissorsResult::Win, RockPaperScissors::Rock) => {
                RockPaperScissors::Paper
            }
            (RockPaperScissorsResult::Loss, RockPaperScissors::Paper) => {
                RockPaperScissors::Rock
            }
            (RockPaperScissorsResult::Draw, RockPaperScissors::Paper) => {
                RockPaperScissors::Paper
            }
            (RockPaperScissorsResult::Win, RockPaperScissors::Paper) => {
                RockPaperScissors::Scissors
            }
            (RockPaperScissorsResult::Loss, RockPaperScissors::Scissors) => {
                RockPaperScissors::Paper
            }
            (RockPaperScissorsResult::Draw, RockPaperScissors::Scissors) => {
                RockPaperScissors::Scissors
            }
            (RockPaperScissorsResult::Win, RockPaperScissors::Scissors) => {
                RockPaperScissors::Rock
            }
        }
    }
}

struct RockPaperScissorsGameGuide {
    rounds : Vec<RockPaperScissorsRoundGuide>,
}

impl FromStr for RockPaperScissorsGameGuide {

    type Err = RockPaperScissorsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let lines = s.lines();
        let mut rounds : Vec<RockPaperScissorsRoundGuide> = Vec::new();

        for line in lines {
            let mut splitted_line = line.split_whitespace();
            let other = RockPaperScissors::try_from(
                splitted_line.next()
                    .ok_or(RockPaperScissorsError::StringParsing)?
                    .chars()
                    .next()
                    .ok_or(RockPaperScissorsError::StringParsing)?)?;
            let own = RockPaperScissorsResult::try_from(
                splitted_line.next()
                    .ok_or(RockPaperScissorsError::StringParsing)?
                    .chars()
                    .next()
                    .ok_or(RockPaperScissorsError::StringParsing)?)?;
            rounds.push(RockPaperScissorsRoundGuide { own, other });
        }

        Ok(RockPaperScissorsGameGuide { rounds })

    }
}



impl From<RockPaperScissorsGameGuide> for RockPaperScissorsGame {
    fn from(guide: RockPaperScissorsGameGuide) -> Self {
        RockPaperScissorsGame {
            rounds : guide.rounds
                .into_iter()
                .map(move |guide| RockPaperScissorsRound::from(guide))
                .collect()
        }
    }
}


impl From<RockPaperScissorsRoundGuide> for RockPaperScissorsRound {
    fn from(guide: RockPaperScissorsRoundGuide) -> Self {
        RockPaperScissorsRound {
            own : guide.play(),
            other : guide.other,
        }
    }
}

enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
enum RockPaperScissorsError {
    #[error("wrong char found")]
    CharParsing,
    #[error("string parsing error")]
    StringParsing,
}

impl TryFrom<char> for RockPaperScissors {
    type Error = RockPaperScissorsError;

    fn try_from(c : char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(RockPaperScissors::Rock),
            'B' => Ok(RockPaperScissors::Paper),
            'C' => Ok(RockPaperScissors::Scissors),
            'X' => Ok(RockPaperScissors::Rock),
            'Y' => Ok(RockPaperScissors::Paper),
            'Z' => Ok(RockPaperScissors::Scissors),
            _ => Err(RockPaperScissorsError::CharParsing),
        }
    }
}

struct RockPaperScissorsRound {
    own : RockPaperScissors,
    other : RockPaperScissors,
}

impl RockPaperScissorsRound {
    fn play(&self) -> RockPaperScissorsResult{
        match (&self.own, &self.other) {
            (RockPaperScissors::Rock, RockPaperScissors:: Rock) => {
                RockPaperScissorsResult::Draw
            }
            (RockPaperScissors::Rock, RockPaperScissors::Paper) => {
                RockPaperScissorsResult::Loss
            }
            (RockPaperScissors::Rock, RockPaperScissors::Scissors) => {
                RockPaperScissorsResult::Win
            }
            (RockPaperScissors::Paper, RockPaperScissors::Rock) => {
                RockPaperScissorsResult::Win
            }
            (RockPaperScissors::Paper, RockPaperScissors::Paper) => {
                RockPaperScissorsResult::Draw
            }
            (RockPaperScissors::Paper, RockPaperScissors::Scissors) => {
                RockPaperScissorsResult::Loss
            }
            (RockPaperScissors::Scissors, RockPaperScissors::Rock) => {
                RockPaperScissorsResult::Loss
            }
            (RockPaperScissors::Scissors, RockPaperScissors::Paper) => {
                RockPaperScissorsResult::Win
            }
            (RockPaperScissors::Scissors, RockPaperScissors::Scissors) => {
                RockPaperScissorsResult::Draw
            }
        }
    }
}

use std::str::FromStr;

struct RockPaperScissorsGame {
    rounds : Vec<RockPaperScissorsRound>,
}

impl FromStr for RockPaperScissorsGame {

    type Err = RockPaperScissorsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let lines = s.lines();
        let mut rounds : Vec<RockPaperScissorsRound> = Vec::new();

        for line in lines {
            let mut splitted_line = line.split_whitespace();
            let other = RockPaperScissors::try_from(
                splitted_line.next()
                    .ok_or(RockPaperScissorsError::StringParsing)?
                    .chars()
                    .next()
                    .ok_or(RockPaperScissorsError::StringParsing)?)?;
            let own = RockPaperScissors::try_from(
                splitted_line.next()
                    .ok_or(RockPaperScissorsError::StringParsing)?
                    .chars()
                    .next()
                    .ok_or(RockPaperScissorsError::StringParsing)?)?;
            rounds.push(RockPaperScissorsRound { own, other });
        }

        Ok(RockPaperScissorsGame { rounds })

    }
}

impl RockPaperScissorsGame {
    fn get_total_score(&self) -> u32 {
        
        let get_score_per_round = |round : &RockPaperScissorsRound| -> u32 {
            let mut score : u32 = 0;
            match round.own {
                RockPaperScissors::Rock => score += 1,
                RockPaperScissors::Paper => score += 2,
                RockPaperScissors::Scissors => score += 3,
            };

            match round.play() {
                RockPaperScissorsResult::Loss => score += 0,
                RockPaperScissorsResult::Draw => score += 3,
                RockPaperScissorsResult::Win => score += 6,
            };

            score
        };

        self.rounds.iter()
            .map(get_score_per_round)
            .fold(0, |n, i| n + i)
    }
}


fn main() {
    let input = include_str!("input");
    {
        let total_score = RockPaperScissorsGame::from_str(input).unwrap()
            .get_total_score();
            println!("Output1: {}", total_score);
    }
    {
        let total_score = RockPaperScissorsGame::from(RockPaperScissorsGameGuide::from_str(input).unwrap())
            .get_total_score();
            println!("Output2: {}", total_score);
    }
    
}

#[test]
fn parse_test01() {
    let test_string = "A Y\nB X\nC Z";
    let total_score = RockPaperScissorsGame::from_str(test_string).unwrap()
        .get_total_score();

    assert_eq!(total_score, 15);        
}

#[test]
fn parse_test02() {
    let test_string = "A Y\nB X\nC Z";
    let total_score = RockPaperScissorsGame::from(RockPaperScissorsGameGuide::from_str(test_string).unwrap())
        .get_total_score();

    assert_eq!(total_score, 12);        
}

