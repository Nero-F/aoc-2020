use std::fs;

fn main() {
    let buffer = fs::read_to_string("input").unwrap();
    let input: Vec<isize> = buffer.lines().map(|x| x.parse().unwrap()).collect();

    let res = part1(&mut input.clone(), 25);
    println!("{}", res);
    println!("{}", part2(input, res));
}

fn is_sum(target: isize, preamble: Vec<isize>) -> bool {
    let mut to_match: isize;

    for i in 0..preamble.len() {
        to_match = target - preamble[i];
        if to_match != preamble[i] && preamble.contains(&to_match) {
            return true;
        }
    } 
    false
}

fn part2(input: Vec<isize>, target: isize) -> isize {
    let mut buff_size = 3;
    let mut buffer:Vec<isize> = input[0..buff_size].to_vec();
    let mut sum: isize;
    let max;
    let min;
    
    loop {
        if buff_size >= input.len() {
            break;
        }
        for i in 0..input.len()-buff_size {
            sum = buffer.iter().sum();
            if sum == target {
                max = *buffer.iter().max().unwrap();
                min = *buffer.iter().min().unwrap();
                return min + max;
            }
            buffer = input[i..i+buff_size].to_vec();
        }
        buff_size += 1;
    }
    panic!("Error!");
}

fn part1(input: &mut Vec<isize>, preamb_len: usize) -> isize {
    let mut preamble: Vec<isize>  = input[0..preamb_len].to_vec();
    let list = input.split_off(preamb_len);

    for nbr in list {
        if !is_sum(nbr, preamble.clone()) {
            return nbr;
        }
        preamble.remove(0);
        preamble.push(nbr);
    }
    panic!("Error!");
}

#[test]
fn atest() {
    let test_input = [ 35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576 ];

    assert_eq!(127, part1(&mut test_input.to_vec(), 5));
    assert_eq!(62, part2(test_input.to_vec(), 127));
}
