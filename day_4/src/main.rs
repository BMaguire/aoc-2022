extern crate common;


// use itertools::Itertools;
use common::file_util;

// use std::collections::HashMap;

fn main() {
    println!("Advent of Code Day 4");
    let data = file_util::read_file("../inputs/day_4.txt", "\n");

    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<String>) {
    println!("Part 1");
    println!("Answer {:?}", find_enveloped_assignments(&process_input(&data)));
}

fn part_2(data: &Vec<String>) {
    println!("Part 2");
    println!("Answer {:?}", find_overlapping_assignments(&process_input(&data)));
}

type Assignment = (usize, usize);
type AssignmentPair = (Assignment, Assignment);

fn process_input(data: &Vec<String>) -> Vec<AssignmentPair> {
    let mut output : Vec<AssignmentPair> = vec![];

    for line in data {
        let mut pair: Vec<Assignment> = vec![];

        for assignment in line.split(",") {
            let range: Vec<&str> = assignment.split("-").collect();

            pair.push((range[0].parse::<usize>().unwrap(), range[1].parse::<usize>().unwrap()));

        }

        output.push((pair[0], pair[1]));
    }

    output
}

fn find_enveloped_assignments(assignment_pairs: &Vec<AssignmentPair>) -> usize {
    assignment_pairs.iter().filter(|assignment_pair| {
            (assignment_pair.0.0 <= assignment_pair.1.0 &&
            assignment_pair.0.1 >= assignment_pair.1.1) || 
            (assignment_pair.1.0 <= assignment_pair.0.0 &&
            assignment_pair.1.1 >= assignment_pair.0.1)
        } 
    ).count()

}

fn find_overlapping_assignments(assignment_pairs: &Vec<AssignmentPair>) ->  usize{

    assignment_pairs.iter().filter(|assignment_pair| {
            (assignment_pair.0.0 <= assignment_pair.1.0 &&
            assignment_pair.0.1 >= assignment_pair.1.1) || 
            (assignment_pair.1.0 <= assignment_pair.0.0 &&
            assignment_pair.1.1 >= assignment_pair.0.1) ||

            (assignment_pair.0.0 >= assignment_pair.1.0 &&
            assignment_pair.0.0 <= assignment_pair.1.1) ||

            (assignment_pair.0.1 >= assignment_pair.1.0 &&
                assignment_pair.0.1 <= assignment_pair.1.1)
        } 
    ).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {

        let input = vec![
            String::from("2-4,6-8"),
            String::from("2-3,4-5"),
            String::from("5-7,7-9"),
            String::from("2-8,3-7"),
            String::from("6-6,4-6"),
            String::from("2-6,4-8")
        ];
        assert_eq!(process_input(&input), vec![
            ((2,4),(6,8)),
            ((2,3),(4,5)),
            ((5,7),(7,9)),
            ((2,8),(3,7)),
            ((6,6),(4,6)),
            ((2,6),(4,8))
        ] );
    }


    #[test]
    fn test_assignment_envelopment() {
        let input = vec![
            String::from("2-4,6-8"),
            String::from("2-3,4-5"),
            String::from("5-7,7-9"),
            String::from("2-8,3-7"),
            String::from("6-6,4-6"),
            String::from("2-6,4-8")
        ];

        assert_eq!(find_enveloped_assignments(&process_input(&input)), 2)
    }


    #[test]
    fn test_assignment_overlap() {
        let input = vec![
            String::from("2-4,6-8"),
            String::from("2-3,4-5"),
            String::from("5-7,7-9"),
            String::from("2-8,3-7"),
            String::from("6-6,4-6"),
            String::from("2-6,4-8")
        ];

        assert_eq!(find_overlapping_assignments(&process_input(&input)), 4)
    }
}
