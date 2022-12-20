use itertools::Itertools;

fn parse_input(input : &str, window_size : usize) -> Option<usize> {
    Some(window_size + input.chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .position(|slice| slice.iter().sorted().dedup().count() == slice.iter().count())?)
}

fn main() {
    let input = include_str!("input");
    println!("Output 1: {}\nOutput 2: {}", parse_input(input,4).unwrap(), parse_input(input,14).unwrap());
}


#[test]
fn test_parser() {
    assert_eq!(parse_input("bvwbjplbgvbhsrlpgdmjqwftvncz",4), Some(5));
    assert_eq!(parse_input("nppdvjthqldpwncqszvftbrmjlhg",4), Some(6));
    assert_eq!(parse_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",4), Some(10));
    assert_eq!(parse_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",4), Some(11));

    assert_eq!(parse_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb",14), Some(19));
    assert_eq!(parse_input("bvwbjplbgvbhsrlpgdmjqwftvncz",14), Some(23));
    assert_eq!(parse_input("nppdvjthqldpwncqszvftbrmjlhg",14), Some(23));
    assert_eq!(parse_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",14), Some(29));
    assert_eq!(parse_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",14), Some(26));
}