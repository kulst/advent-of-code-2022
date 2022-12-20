use std::collections::HashMap;

fn parse_start_situation(s : &str) -> (HashMap<usize,Vec<char>>,Vec<usize>) {
    let mut lines = s.lines().rev();
    let indices = lines.next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut result = HashMap::from_iter(
        indices.iter().cloned().zip(std::iter::repeat(Vec::new()))
    );

    for line in lines {
        for (char, index) in line.chars().skip(1).step_by(4).zip(&indices) {
            if char.is_ascii_alphabetic() {
                result.get_mut(index).unwrap().push(char);
            }
        }
    }
    (result, indices)
}

fn parse_commands(s : &str) -> Vec<(usize, usize, usize)> {
    s.lines()
        .map(|s| {
            s.split_whitespace()
                .filter(|s| !s.contains(char::is_alphabetic))
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|entry| (entry[0], entry[1], entry[2]))
        .collect::<Vec<_>>()
}

fn move_crates(map : &mut HashMap<usize, Vec<char>>, commands : &[(usize, usize, usize)]) {
    for (count, from, to) in commands {
        for _ in 1..=*count {
            let val = {map.get_mut(from).unwrap().pop().unwrap()};
            map.get_mut(to).unwrap().push(val);
        }
    }
}

fn move_crates_in_order(map : &mut HashMap<usize, Vec<char>>, commands : &[(usize, usize, usize)]) {
    for (count, from, to) in commands {
        let mut vals = Vec::new();
        for _ in 1..=*count {
            vals.push(map.get_mut(from).unwrap().pop().unwrap());
        }
        for _ in 1..=*count {
            map.get_mut(to).unwrap().push(vals.pop().unwrap());
        }
    }
}

fn get_code(map : &HashMap<usize, Vec<char>>, indices : &[usize]) -> Vec<(usize, char)> {
    let mut result = Vec::new();
    for index in indices {
        result.push((*index,map.get(index).unwrap().last().unwrap().clone()));
    }
    result
}




fn main() {
    let input = include_str!("input");
    let mut blocks1 = input.split("\n\n");
    let mut blocks2 = input.split("\n\n");
    let (mut map1, indices1) = parse_start_situation(blocks1.next().unwrap());
    let commands1 = parse_commands(blocks1.next().unwrap());

    let (mut map2, indices2) = parse_start_situation(blocks2.next().unwrap());
    let commands2 = parse_commands(blocks2.next().unwrap());
    move_crates(&mut map1, &commands1);
    move_crates_in_order(&mut map2, &commands2);

    println!("Output1: {:?}\nOutput2: {:?}", get_code(&map1, &indices1), get_code(&map2, &indices2));

}

#[test]
fn test_parse_start() {
    let test_string = "    [D]    \n\
        [N] [C]    \n\
        [Z] [M] [P]\n\
        1   2   3";

    assert_eq!(parse_start_situation(test_string),
        (HashMap::from([  (1,vec!['Z','N']),(2,vec!['M','C','D']),(3,vec!['P'])   ]),vec![1usize,2,3]));

}

#[test]
fn test_parse_commands() {
    let test_string = "move 1 from 2 to 1\n\
        move 3 from 1 to 3\n\
        move 2 from 2 to 1\n\
        move 1 from 1 to 2";

    assert_eq!(parse_commands(test_string),
        vec![(1,2,1),(3,1,3),(2,2,1),(1,1,2)]);

}

#[test]
fn test_move_crates() {
    let test_string = "    [D]    \n\
        [N] [C]    \n\
        [Z] [M] [P]\n\
        1   2   3 \n\
        \n\
        move 1 from 2 to 1\n\
        move 3 from 1 to 3\n\
        move 2 from 2 to 1\n\
        move 1 from 1 to 2";

    let mut blocks = test_string.split("\n\n");
    let (mut map, indices) = parse_start_situation(blocks.next().unwrap());
    let commands = parse_commands(blocks.next().unwrap());

    move_crates(&mut map, &commands);

    assert_eq!(map,
        HashMap::from([(1,vec!['C']),(2,vec!['M']),(3,vec!['P','D','N','Z'])]));

    assert_eq!(get_code(&map, &indices),vec![(1,'C'),(2,'M'),(3,'Z')]);
    
}

#[test]
fn test_move_crates_in_order() {
    let test_string = "    [D]    \n\
        [N] [C]    \n\
        [Z] [M] [P]\n\
        1   2   3 \n\
        \n\
        move 1 from 2 to 1\n\
        move 3 from 1 to 3\n\
        move 2 from 2 to 1\n\
        move 1 from 1 to 2";

    let mut blocks = test_string.split("\n\n");
    let (mut map, indices) = parse_start_situation(blocks.next().unwrap());
    let commands = parse_commands(blocks.next().unwrap());

    move_crates_in_order(&mut map, &commands);

    assert_eq!(map,
        HashMap::from([(1,vec!['M']),(2,vec!['C']),(3,vec!['P','Z','N','D'])]));

    assert_eq!(get_code(&map, &indices),vec![(1,'M'),(2,'C'),(3,'D')]);
    
}