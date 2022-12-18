use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(PartialEq, Eq, Debug)]
struct Elf {
    food_calories : Vec<u32>,
}
#[derive(PartialEq, Eq, Debug)]
struct ElfTeam {
    members : Vec<Elf>,
}

#[derive(Error, Debug, PartialEq, Eq)]
enum ParseElfTeamError {
    #[error("parsing to int not possible")]
    Parsing(#[from] ParseIntError),
}

impl FromStr for ElfTeam {
    type Err = ParseElfTeamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string_elves : Vec<&str> = s
            .split("\n\n")
            .collect();
        
        let mut elves = ElfTeam {
            members : Vec::with_capacity(string_elves.len()),
        };
        
        for string_elf in string_elves {
            let elf = Elf {
                food_calories : string_elf
                    .lines()
                    .map(|val| Ok(val.parse()?))
                    .collect::<Result<Vec<u32>, ParseElfTeamError>>()?,
            };
            elves.members.push(elf);
        }
    
        Ok(elves)
    }
        
}

impl ElfTeam {
    fn get_calory_sums(&self) -> BTreeSet<u32> {
        self.members
            .iter()
            .map(|elf| elf.food_calories.iter().sum())
            .collect()
    }

    fn get_max_calory_sum(&self) -> u32 {
        match self.get_calory_sums().iter().max() {
            Some(val) => *val,
            _ => u32::MIN,
        }
    }

    fn get_largest_calories_sum(&self, count : usize) -> u32 {
        self.get_calory_sums().iter().rev().take(count).sum()
    }
}


fn main () {
    let input = include_str!("input");
    let team = ElfTeam::from_str(input).unwrap();
    println!("Output1: {}", team.get_max_calory_sum());
    println!("Output2: {}", team.get_largest_calories_sum(3));
}

#[test]
fn parse_test_string() {
    let test_string = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    let elf1 = Elf {
        food_calories : vec![1000, 2000, 3000],
    };
    let elf2 = Elf {
        food_calories : vec![4000],
    };
    let elf3 = Elf {
        food_calories : vec![5000, 6000],
    };
    let elf4 = Elf {
        food_calories : vec![7000, 8000, 9000],
    };
    let elf5 = Elf {
        food_calories :  vec![10000],
    };
    let elves = ElfTeam {
        members : vec![elf1, elf2, elf3, elf4, elf5]
    };

    let test_elves : ElfTeam = ElfTeam::from_str(test_string).unwrap();

    assert_eq!(elves, test_elves);

    assert_eq!(BTreeSet::from([6000, 4000, 11000, 24000, 10000]), test_elves.get_calory_sums());

    assert_eq!(24000, test_elves.get_max_calory_sum());

    assert_eq!(45000, test_elves.get_largest_calories_sum(3));

}




