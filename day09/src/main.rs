use std::fs;
use std::collections::HashMap;

fn main() {
    let buffer = fs::read_to_string("input").unwrap();
    let mut input: Vec<isize> = buffer.lines().map(|x| x.parse().unwrap()).collect();
    println!("{}", part1(&mut input));
    println!("{}", part2(&input));
}

fn part1(input: &mut Vec<isize>) -> isize {
    let mut ejr: isize = 0;
    let mut tmp: isize = 0;
    let mut differencies: (isize, isize) = (0, 0);

    input.sort();
    input
        .iter()
        .for_each(|joltage| {
            tmp = joltage - ejr;
            match tmp {
                1 => differencies.0 += 1,
                3 => differencies.1 += 1,
                _ => {}
            };
            ejr = *joltage;
        });
    differencies.0 * (1 + differencies.1)
}

fn part2(input: &Vec<isize>) -> isize {
    let mut dp: HashMap<isize, isize>= HashMap::new();
    dp.insert(0, 1);
    let mut tmp: isize= 0;

    for adapter in input {
        for j in 1..=3 {
            match dp.get(&(adapter - j)) {
                Some(x) => tmp += x,
                None => (),
            };
        }
        dp.insert(*adapter, tmp);
        tmp = 0;
    };
    *dp.get(input.iter().max().unwrap()).unwrap()
}

#[test]
fn atest() {
    let mut test_input1 = [28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3].to_vec();
    let mut test_input2: Vec<isize> = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4].to_vec();
    test_input2.sort();
    assert_eq!(220, part1(&mut test_input1));
    assert_eq!(8, part2(&test_input2));
    assert_eq!(19208, part2(&test_input1));
}

//fn test_part2() {
    //let test_input = [
        //"(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49, (52)",
        //"(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 49, (52)",
        //"(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46, 48, 49, (52)",
        //"(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46, 49, (52)",
        //"(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 47, 48, 49, (52)",
        //"(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 46, 48, 49, (52)",
        //"(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 46, 49, (52)",
        //"(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 47, 48, 49, (52)",
        //"(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 47, 49, (52)",
        //"(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 48, 49, (52)"
    //];
    //assert_eq!(19208, part2(test_input.to_vec()));
//}
