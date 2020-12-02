use std::fs;

struct Element {
    pub nbrs: (usize, usize),
    pub target: char,
    pub password: String,
}

fn main() {
    let buffer = fs::read_to_string("input").unwrap();
    let input: Vec<&str> = buffer.lines().collect();
    println!("{}", part1(input.clone()));
    println!("{}", part2(input));
}

fn line_to_element(line: &str) -> Element
{
    let buffer: Vec<&str>= line.split(" ").collect();
    let boundaries: Vec<usize> = buffer[0].split("-").collect::<Vec<&str>>().iter().map(|&x| { x.parse().unwrap() }).collect();
    Element {
        nbrs: (boundaries[0], boundaries[1]),
        target: buffer[1].chars().nth(0).unwrap(),
        password: String::from(buffer[2])
    }
}

fn part1(input: Vec<&str>) -> i32
{
    let mut counter = 0;
    let mut occurences;
    
    for i in input {
        let element = line_to_element(i);
        occurences = element.password.matches(element.target).count();

        if occurences >= element.nbrs.0 && occurences <= element.nbrs.1 {
            counter += 1;
        }
    }
    counter
}

fn part2(input: Vec<&str>) -> i32
{
    let mut counter = 0;
    let mut first;
    let mut second;
    
    for i in input {
        let element = line_to_element(i);
        first = element.password.chars().nth(element.nbrs.0 - 1).unwrap();
        second = element.password.chars().nth(element.nbrs.1 - 1).unwrap();

        if element.target == first && element.target != second  || element.target != first && element.target == second  {
            counter += 1;
        }
    }
    counter
}

#[test]
fn automated_tests() {
    assert_eq!(2 , part1(["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"].to_vec()));
    assert_eq!(2 , part2(["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"].to_vec()));

}
