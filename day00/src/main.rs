use std::fs;

fn main() {
    let buffer = fs::read_to_string("input").unwrap();
    let input: Vec<i32> = buffer.lines().map(|x| x.parse().unwrap()).collect();
    
    println!("{:?}", part1(input.clone()));
    println!("{:?}", part2(input.clone()));
}

fn part1(entries: Vec<i32>) -> i32 { 
    let mut tmp = Vec::new();
    let mut buffer: i32;

    for entry in entries {
        buffer = 2020 - entry;
        if tmp.contains(&buffer) {
            return buffer * entry
        } else {
            tmp.push(entry);
        }
    }
    panic!("Error")
}
fn part2(mut entries: Vec<i32>) -> i32 {
    entries.sort();
    let mut r;
    let mut l;
    let mut curr_sum;
    let mut i = 0;

    while i <= entries.len() - 2 {
        r = entries.len() - 1;
        l = i + 1;
        while r > l {
            curr_sum = entries[l] + entries[i] + entries[r];
            if curr_sum == 2_020 {
                return entries[l] * entries[i] * entries[r]
            }
            if curr_sum < 2_020 {
                l += 1;
            } else if curr_sum > 2_020 {
                r -= 1;
            }
        }
        i += 1;
    }
    panic!("Error")
}


#[test]
fn automated_tests() {
    assert_eq!(514579 , part1([1721, 979, 366, 299, 675, 1456].to_vec()));
    assert_eq!(241861950 , part2([1721, 979, 366, 299, 675, 1456].to_vec()));
}
