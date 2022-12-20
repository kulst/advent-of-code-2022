use std::collections::HashMap;

fn parse_start_situation(s : &str) -> HashMap<usize,Vec<char>> {
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
    result
}




fn main() {


}

#[test]
fn test_parse_start() {
    let test_string = "    [D]    \n\
        [N] [C]    \n\
        [Z] [M] [P]\n\
        1   2   3";

    assert_eq!(parse_start_situation(test_string),
        HashMap::from([  (1,vec!['Z','N']),(2,vec!['M','C','D']),(3,vec!['P'])   ]));

}