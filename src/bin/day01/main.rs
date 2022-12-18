use std::str::FromStr;

struct Elf {
    FoodCalories : Vec<u32>,
}

struct ElfTeam {
    Members : Vec<Elf>,
}

struct ParseElfTeamError;

impl FromStr for ElfTeam {
    type Err = ParseElfTeamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut string_elves : Vec<&str> = s
            .split("\n\n")
            .collect();
        
        let mut elves = Vec::new();

        for string_elf in string_elves {
            let mut elf = string_elf
                            .lines()
                            .map(str::parse<u32>)
                            .collect();

        }
        
    }
}

fn parse 


fn main () {
    // entscheiden ob puzzle 01 oder 02
    
    // string einlesen

    // string.split("\n\n")
    //       .lines()
    //       .

    
}



