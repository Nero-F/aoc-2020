use std::fs;

fn main() {
    let tmp = fs::read_to_string("input").unwrap();
    let input: Vec<&str>= tmp.lines().collect();

    println!("{}", part1(input.clone()));
    println!("{}", part2(input));
}

fn get_nb_questions(questions: Vec<&str>) -> usize {
    let mut clean: Vec<char> = String::from(questions.join("")).chars().collect();
    clean.sort();
    clean.dedup();
    clean.len()
}

fn part1(input: Vec<&str>) -> usize {
    let mut tmp: Vec<&str> = Vec::new();
    let mut res = 0;

    for i in 0..input.len() {
        if input[i].len() == 0 {
            res += get_nb_questions(tmp.clone());
            tmp.clear();
        } else {
            tmp.push(input[i]);
        }
    }
    return res + get_nb_questions(tmp.clone());
}

fn get_yes_questions(input: Vec<&str>) -> usize {
    if input.len() == 1 {
        return input[0].len();
    }
    let mut clean: Vec<char> = String::from(input.join("")).chars().collect();
    clean.sort();
    let poeple = input.len();
    let mut count = 0;
    let mut tmp: Vec<char> = Vec::new();

    for letter in clean {
        if tmp.contains(&letter) || tmp.is_empty() == true {
            tmp.push(letter);
            if tmp.len() == poeple {
                count += 1;
            }
        } else  {
            tmp.clear();
            tmp.push(letter);
        }
    }
    count
}

fn part2(input: Vec<&str>) -> usize {
    let mut tmp: Vec<&str> = Vec::new();
    let mut res = 0;
    let mut foo = 0;

    for i in input {
        if i.len() == 0 {
            foo = get_yes_questions(tmp.clone());
            res += foo;
            tmp.clear();
        } else {
            tmp.push(i);
        }
    }
    return res + get_yes_questions(tmp.clone());
}

#[test]
fn  atests_part1() {
    let test_input = [
        "abc",
        "",
        "a",
        "b",
        "c",
        "",
        "ab",
        "ac",
        "",
        "a",
        "a",
        "a",
        "a",
        "",
        "b"
    ];
    assert_eq!(11, part1(test_input.to_vec()));
    assert_eq!(6, part2(test_input.to_vec()));
}
