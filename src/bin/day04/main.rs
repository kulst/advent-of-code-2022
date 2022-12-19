use interval::interval::Interval;
use interval::ops::Range;
use gcollections::ops::set::{Subset,Overlap};

fn parse_input(s : &str) -> Vec<(Interval<usize>, Interval<usize>)> {
    s.lines()
        .map(|line| {
            let numbers = line.split(&[',','-'])
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (Interval::new(numbers[0], numbers[1]), Interval::new(numbers[2], numbers[3]))
        })
        .collect()
}

fn check_range_sets(slice : &[(Interval<usize>, Interval<usize>)]) -> (usize, usize) {  
    (
        slice.iter()
            .filter(|(first, second)| first.is_subset(second) || second.is_subset(first))
            .count(),
        slice.iter()
            .filter(|(first, second)| first.overlap(second))
            .count()
    )
}

fn main() {
    let input = include_str!("input");
    let output = check_range_sets(&parse_input(input));
    println!("Output1: {}\nOutput2: {}", output.0, output.1);
}

#[test]
fn test_parse_input() {
    let test_string = "2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8";
    
    assert_eq!(parse_input(test_string),
        vec![   (Interval::new(2,4), Interval::new(6, 8)),
                (Interval::new(2,3), Interval::new(4, 5)),
                (Interval::new(5,7), Interval::new(7, 9)),
                (Interval::new(2,8), Interval::new(3, 7)),
                (Interval::new(6,6), Interval::new(4, 6)),
                (Interval::new(2,6), Interval::new(4, 8)),        
        ]);
}

#[test]
fn test_check_ranges() {
    let test_string = "2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8";

    assert_eq!(check_range_sets(&parse_input(test_string)),(2,4));
}