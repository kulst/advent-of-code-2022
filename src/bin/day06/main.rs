use itertools::Itertools;

fn find_marker(datastream : &str, marker_size : usize) -> Option<usize> {
    Some(marker_size + datastream.chars()
        .collect::<Vec<char>>()
        .windows(marker_size)
        .position(|window| window.iter().sorted().dedup().count() == window.iter().count())?)
}

fn main() {
    let input = include_str!("input");
    println!("Output 1: {}\nOutput 2: {}", find_marker(input,4).unwrap(), find_marker(input,14).unwrap());
}


#[test]
fn test_parser() {
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz",4), Some(5));
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg",4), Some(6));
    assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",4), Some(10));
    assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",4), Some(11));

    assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb",14), Some(19));
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz",14), Some(23));
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg",14), Some(23));
    assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",14), Some(29));
    assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",14), Some(26));
}