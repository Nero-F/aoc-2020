use std::fs;

fn main() {
    let buffer = fs::read_to_string("input").unwrap();
    let input: Vec<&str> = buffer.lines().collect();
    println!("{}", part1(input.clone()));
    println!("{}", part2(input));
}


fn check_slope(input: Vec<&str>, slopes: (usize, usize)) -> usize {
    let mut element;
    let mut counter = 0;
    let mut x = slopes.0;
    let mut y = slopes.1;
    let width = input[0].len();

    while y < input.len() {
        element = input[y].chars().nth(x % width).unwrap();
        if element == '#' {
            counter += 1
        }
        x += slopes.0;
        y += slopes.1;
    }
    counter
}

fn part1(input: Vec<&str>) -> usize {
    let res: usize = check_slope(input, (3, 1));
    res
}

fn part2(input: Vec<&str>) -> usize {
    let slopes_tab = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].to_vec();
    let mut res: usize = 1;
    
    for slope in slopes_tab {
        res *= check_slope(input.clone(), slope);
    }
    res
}

#[test]
fn automated_tests() {
    let test_input = [
        "..##.........##.........##.........##.........##.........##.......",
        "#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..",
        ".#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.",
        "..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#",
        ".#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.",
        "..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....",
        ".#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#",
        ".#........#.#........#.#........#.#........#.#........#.#........#",
        "#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...",
        "#...##....##...##....##...##....##...##....##...##....##...##....#",
        ".#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#"
    ];
    assert_eq!(7 , part1(test_input.to_vec()));
    assert_eq!(336 , part2(test_input.to_vec()));
}
