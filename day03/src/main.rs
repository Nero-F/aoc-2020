#[macro_use] extern crate lazy_static;
extern crate regex;
use std::fs;
use regex::Regex;

fn main() {
    let buffer = fs::read_to_string("input").unwrap();
    let input: Vec<&str> = buffer.lines().collect();

    println!("{}", part1(input.clone()));
    println!("{}", part2(input));
}

fn is_post(pass: [bool; 8]) -> usize {
    for b in pass.iter() {
        if *b == false {
            return 0;
        }
    }
    1
}

fn parse_passport(line: &str, instruction: [&str; 8], passport: &mut [bool; 8]) {
    let tab_elem = line.split(" ").collect::<Vec<&str>>();
    let mut data: Vec<&str>;

    for elem in tab_elem {
        data = elem.split(":").collect();
        match instruction.iter().position(|x| *x == data[0]) {
            Some(position) => { passport[position] = true; },
            None => {  }
        };
    }
}

fn part1(input: Vec<&str>) -> usize {
    let instruction = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    let mut passport: [bool; 8] = [ false, false, false, false, false, false, false, true ];
    let mut counter: usize = 0;

    for line in input {
        if line.len() == 0  {
            counter += is_post(passport);
            passport = [ false, false, false, false, false, false, false, true ];
        } else {
            parse_passport(line, instruction, &mut passport);
        }
    }
    counter += is_post(passport);
    counter
}

fn parse_passport2(line: &str, instruction: [&str; 8], passport: &mut [bool; 8]) {
    let tab_elem = line.split(" ").collect::<Vec<&str>>();
    let mut data: Vec<&str>;

    for elem in tab_elem {
        data = elem.split(":").collect();
        match data[0] {
            "byr" => { 
                let nb: usize = data[1].parse().unwrap();
                if data[1].len() == 4 && nb >= 1920 && nb <= 2002 { passport[instruction.iter().position(|&s| s == data[0]).unwrap()] = true;  }
            },
            "iyr" => { 
                let nb: usize = data[1].parse().unwrap();
                if data[1].len() == 4 && nb >= 2010 && nb <= 2020 { passport[instruction.iter().position(|&s| s == data[0]).unwrap()] = true;  }
            },
            "eyr" => { 
                let nb: usize = data[1].parse().unwrap();
                if data[1].len() == 4 && nb >= 2020 && nb <= 2030 { passport[instruction.iter().position(|&s| s == data[0]).unwrap()] = true;  }
            },
            "hgt" => { 
                lazy_static! {
                    static ref REGX: Regex = Regex::new("^(1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in)$").unwrap();
                }
                if REGX.is_match(data[1]) { passport[instruction.iter().position(|&s| s == data[0]).unwrap()] = true;  }},
            "hcl" => { 
                lazy_static! {
                    static ref REGX: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
                }
                if REGX.is_match(data[1]) { passport[instruction.iter().position(|&s| s == data[0]).unwrap()] = true;  }
            },
            "ecl" => { 
                lazy_static! {
                    static ref REGX: Regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
                }
                if REGX.is_match(data[1]) { passport[instruction.iter().position(|&s| s == data[0]).unwrap()] = true;  }},
            "pid" => { 
                lazy_static! {
                    static ref REGX: Regex = Regex::new("^[0-9]{9}$").unwrap();
                }
                if REGX.is_match(data[1]) { passport[instruction.iter().position(|&s| s == data[0]).unwrap()] = true;  }},
            "cid" => {},
            &_ => {}
        }
    }
}

fn part2(input: Vec<&str>) -> usize {
    let instruction = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    let mut passport: [bool; 8] = [ false, false, false, false, false, false, false, true ];
    let mut counter: usize = 0;

    for line in input {
        if line.len() == 0  {
            counter += is_post(passport);
            passport = [ false, false, false, false, false, false, false, true ];
        } else {
            parse_passport2(line, instruction, &mut passport);
        }
    }
    counter += is_post(passport);
    counter
}

#[test]
fn automated_tests() {
    let test_input = [
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
        "byr:1937 iyr:2017 cid:147 hgt:183cm",
        "",
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
        "hcl:#cfa07d byr:1929",
        "",
        "hcl:#ae17e1 iyr:2013",
        "eyr:2024",
        "ecl:brn pid:760753108 byr:1931",
        "hgt:179cm",
        "",
        "hcl:#cfa07d eyr:2025 pid:166559648",
        "iyr:2011 ecl:brn hgt:59in"
    ];
    let test_input1 = [
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
        "hcl:#623a2f",
        "",
        "eyr:2029 ecl:blu cid:129 byr:1989",
        "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        "",
        "hcl:#888785",
        "hgt:164cm byr:2001 iyr:2015 cid:88",
        "pid:545766238 ecl:hzl",
        "eyr:2022",
        "",
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
    ];

    let test_input2 = [
        "eyr:1972 cid:100",
        "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        "",
        "iyr:2019",
        "hcl:#602927 eyr:1967 hgt:170cm",
        "ecl:grn pid:012533040 byr:1946",
        "",
        "hcl:dab227 iyr:2012",
        "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        "",
        "hgt:59cm ecl:zzz",
        "eyr:2038 hcl:74454a iyr:2023",
        "pid:3556412378 byr:2007"
    ];
    assert_eq!(2 , part1(test_input.to_vec()));
    assert_eq!(4 , part2(test_input1.to_vec()));
    assert_eq!(0 , part2(test_input2.to_vec()));
}
