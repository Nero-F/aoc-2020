use std::collections::HashMap;
use std::fs;

fn main() {
    let buffer = fs::read_to_string("input").unwrap();
    let hash_map = map_converter(&*buffer);
    println!("{}", part1(&hash_map));
    println!("{}", part2(&hash_map));
}

struct BagContainer {
    bag_name: String,
    num: usize,
}

impl BagContainer {
    fn recurive_search(&self, bag: &str, hash_map: &HashMap<String, Vec<BagContainer>>) -> bool {
        if self.bag_name == bag {
            return true;
        }
        hash_map.get(&self.bag_name)
            .unwrap()
            .iter()
            .any(|br| br.recurive_search(bag, hash_map))
    }

    fn counter(&self, hash_map: &HashMap<String, Vec<BagContainer>>, prev_count: usize) -> usize {
        let rules = hash_map.get(&self.bag_name).unwrap();
        if rules.is_empty() {
            prev_count
        } else {
            rules.iter()
                .map(|br| br.counter(hash_map, br.num * prev_count))
                .sum::<usize>()
                + prev_count
        }
    }
}

impl From<&str> for BagContainer {
    fn from(s: &str) -> Self {
        match s.find(" ") {
            Some(n) => {
                let num: usize = s[0..n].parse().unwrap();
                BagContainer {
                    num,
                    bag_name: String::from(s[n + 1..].trim_end_matches("s"))
                }
            }
            None => { panic!("boom") }
        }
    }
}

fn map_converter(input: &str) -> HashMap<String, Vec<BagContainer>> {
    input.lines()
        .map(|i| {
            let mut splt = i.split(" contain ");
            let bag = splt.next().unwrap().trim_end_matches("s");
            let unparsed_rules = splt.next().unwrap().trim_end_matches(".");
            let rules: Vec<BagContainer> = if unparsed_rules == "no other bags" {
                vec![]
            } else {
                unparsed_rules.split(", ").map(|s| s.into()).collect()
            };
            (String::from(bag), rules)
        }).collect()
}

fn part1(input: &HashMap<String, Vec<BagContainer>>) -> usize {
    input.iter()
        .filter(|(bag, rules)| {
            if bag.as_str() == "shiny gold bag" {
                false
            } else {
                rules
                    .iter()
                    .any(|br| br.recurive_search("shiny gold bag", input))
            }
        }).count()
}

fn part2(input: &HashMap<String, Vec<BagContainer>>) -> usize {
    let rules = input.get("shiny gold bag").unwrap();
    return rules
        .iter()
        .map(|br| br.counter(input, br.num))
        .sum::<usize>();
}

#[test]
fn atests_part1() {
    let test_input = fs::read_to_string("test_input").unwrap();
    let hash_map = map_converter(&test_input);
    assert_eq!(4 , part1(&hash_map));
}

