use thiserror::Error;


#[derive(Error, Debug, PartialEq, Eq)]
enum BackpackingError {
    #[error("wrong char found")]
    WrongChar,
    #[error("len of the line is not even")]
    LineLenNotEven,
    #[error("group data not plausible")]
    GroupDataNotPlausible,
    #[error("input slice data not plausible")]
    SliceDataNotPlausible,
}

fn get_priority_from_char(c : &char) -> std::result::Result<u32, BackpackingError> {
    match *c {
        'a'..='z' => Ok(u32::from(*c) - (u32::from('a') - 1)),
        'A'..='Z' => Ok(u32::from(*c) - (u32::from('A') - 27)),
        _ => Err(BackpackingError::WrongChar),
    }
}

use itertools::Itertools;

fn get_priorities_from_line(slice : &str) -> std::result::Result<u32, BackpackingError> {
    if slice.len() % 2 != 0 {
        return Err(BackpackingError::LineLenNotEven);
    }
    let (slice1, slice2) = slice.split_at(slice.len()/2);
    slice1.chars()
        .filter(|c| slice2.contains(*c))
        .dedup()
        .map(|c| get_priority_from_char(&c))
        .try_fold(0, |n, val| {
            match val {
                Ok(i) => Ok(n + i),
                Err(err) => Err(err),
            }
        })
}

fn get_badge_from_group(lines : &[&str]) -> Result<char, BackpackingError> {
    if lines.len() != 3 {
        return Err(BackpackingError::GroupDataNotPlausible);
    }

    let chars : Vec<char> = lines[0]
        .chars()
        .filter(|c| lines[1].contains(*c) && lines[2].contains(*c))
        .dedup()
        .collect();
    
    if chars.len() != 1 {
        return Err(BackpackingError::GroupDataNotPlausible);
    }

    Ok(chars[0])
}

fn get_badges_from_groups(slice : &str) -> Result<Vec<char>, BackpackingError> {
    let lines : Vec<&str> = slice.lines().collect();

    if !(lines.len() % 3 == 0) {
        return Err(BackpackingError::SliceDataNotPlausible);
    }

    lines.chunks(3)
        .map(|slice| get_badge_from_group(slice))
        .collect()

}

fn main() {
    let input = include_str!("input");

    let priority_sum : u32 = input.lines()
        .map(|line| get_priorities_from_line(line).unwrap())
        .sum();

    let priority_groups_sum : u32 = get_badges_from_groups(input).unwrap()
        .iter()
        .map(|c| get_priority_from_char(c))
        .try_fold(0, |n, val| {
            match val {
                Ok(i) => Ok(n + i),
                Err(err) => Err(err),
            }
        }).unwrap();


    println!("Output1: {}", priority_sum);
    println!("Output2: {}", priority_groups_sum);
}

#[test]
fn test_priority_from_char() {
    for (index, c) in ('a'..='z').enumerate() {
        assert_eq!(index as u32 + 1, get_priority_from_char(&c).unwrap());
    }

    for (index, c) in ('A'..='Z').enumerate() {
        assert_eq!(index as u32 + 27, get_priority_from_char(&c).unwrap());
    }

    assert!(get_priority_from_char(&'@').is_err());
}

#[test]
fn test_priorities_from_line() {
    let test_str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(test_str.lines()
        .map(|line| get_priorities_from_line(line).unwrap())
        .collect::<Vec<u32>>(),
        vec![16, 38, 42, 22, 20, 19]);
}

#[test]
fn test_badges_from_groups() {
    let test_str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(get_badges_from_groups(test_str).unwrap(), vec!['r', 'Z']); 
}
