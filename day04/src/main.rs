use std::str;
use std::fs;

fn main() {
    let buffer = fs::read_to_string("input").unwrap();
    let input: Vec<&str> = buffer.lines().collect();
    println!("{}", part1(input.clone()));
    println!("{}", part2(input));
}

fn bps_to_id(entry: &str) -> isize {
    let bin = str::replace(entry, "F", "0").replace("B", "1").replace("L", "0").replace("R", "1");
    let value = isize::from_str_radix(&bin, 2).unwrap();
    value
}

fn part1(input: Vec<&str>) -> isize {
    let mut best: isize = 0;
    let mut tmp;

    for entry in input {
        tmp = bps_to_id(entry);
        if tmp > best {
            best = tmp;
        }
    }
   best 
}

fn part2(input: Vec<&str>) -> usize {
    let mut tmp: usize;
    let mut tab: Vec<usize> = Vec::new();

    for entry in input {
        tmp = bps_to_id(entry) as usize;
        tab.push(tmp);
    }
    tab.sort();
    let mut lowest = 0;
    let mut highest = tab.len() - 1;
    let mut mid: usize;

    while highest - lowest >= 2 {
        mid = (lowest + highest) / 2;
        if mid - lowest != tab[mid] - tab[lowest] {
            highest = mid;
        } else {
            lowest = mid + 1;
        }
    }
    if lowest == tab[lowest] - tab[0] { highest + tab[0] } else { lowest + tab[0] }
}

#[test]
fn atests_part1() {
    assert_eq!(567 , bps_to_id("BFFFBBFRRR"));
    assert_eq!(119 , bps_to_id("FFFBBBFRRR"));
    assert_eq!(820 , bps_to_id("BBFFBBFRLL"));
}
