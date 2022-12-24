extern crate common;


// use itertools::Itertools;
use common::file_util;

use std::collections::HashMap;

fn main() {
    println!("Advent of Code Day 3");
    let data = file_util::read_file("../inputs/day_3.txt", "\n");

    part_1(&data);
    part_2(&data);
}

type Bag = (String, String);

fn part_1(data: &Vec<String>) {
    println!("Part 1");
    println!("Answer {:?}", sum_missing_priorities(&process_input(&data)));
}

fn part_2(data: &Vec<String>) {
    println!("Part 2");
    println!("Answer {:?}", sum_groups(&data));
}

fn process_input(data: &Vec<String>) -> Vec<Bag> {
    data.iter().map( |line| split_bag(&line) ).collect::<Vec<Bag>>()
}

fn split_bag(line: &str) -> Bag {
    (String::from(&line[0..line.len()/2]), String::from(&line[line.len()/2..line.len()]))
}

fn sum_missing_priorities(bag_set: &Vec<Bag>) -> usize {
    bag_set.iter().map(|bag|  find_missing( &bag ) ).sum()
}

fn find_missing(bag: &Bag) -> usize {
    for item in bag.0.split("") {
        for other in bag.1.split("") {
            if (item == other) && (item != "") && (other != "") { 
                return letter_to_priority(other)
            }
        }
    }
    0
}


fn sum_groups(bags: &Vec<String>) -> usize {
    
    bags.chunks(3).map(|set| compare_group((*set).to_vec()) ).sum::<usize>()
}

fn compare_group(group: Vec<String>) -> usize {
    
    // let a = group.iter().map(|bag| bag.split("").filter(|&a| a != "").collect::<Vec<_>>());
    let bags = &group[1..group.len()];
    let compare = &group[0];

    
    let item = compare.split("").filter(|&a| a != "").filter(|item| {
        for local_bag in bags {
            match local_bag.find(item) {
                Some(_) => {true; },
                None => {return false; }
            }
        };
        true
    }).collect::<Vec<_>>();

    let key = String::from(*item.first().unwrap());

    letter_to_priority(key.as_ref())
}

fn letter_to_priority(letter: &str) -> usize {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".find(letter).unwrap() + 1
}

// fn find

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing() {
        assert_eq!(find_missing(&split_bag("vJrwpWtwJgWrhcsFMMfFFhFp")), 16);
    }

    #[test]
    fn test_letter_to_priority() {
        assert_eq!(letter_to_priority("p"), 16);
    }

    #[test]
    fn test_compare_group() {
        let input = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg")
        ];
        assert_eq!(compare_group(input), 18);
    }

}
