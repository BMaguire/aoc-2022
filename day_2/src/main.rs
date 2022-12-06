extern crate common;


// use itertools::Itertools;
use common::file_util;

use std::collections::HashMap;

fn main() {
    println!("Advent of Code Day 2");
    let data = file_util::read_file("../inputs/day_2.txt", "\n");

    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<String>) {
    println!("Part 1");

    // calculate the number of times a depth measurement increases
    println!("Answer {:?}", run_strategy_1(&process_strategy(&data)));
}

fn part_2(data: &Vec<String>) {
    println!("Part 2");
    println!("Answer {:?}", run_strategy_2(&process_strategy(&data)));
}


fn process_strategy(data: &Vec<String>) -> Vec<(&str, &str)>{

    data.into_iter().map( |value| { 
        let a = value.split(" ").into_iter().collect::<Vec<&str>>(); (a[0], a[1]) 
    } ).collect()
}

fn run_strategy_2(strategies: &Vec<(&str, &str)>) -> u64 {
    // map values to results

    let term_map = HashMap::from([
        ("A", &RPS::ROCK),
        ("B", &RPS::PAPER),
        ("C", &RPS::SCISSORS)
    ]);

    let result_map = HashMap::from([
        ("X", &RPSresult::LOSE),
        ("Y", &RPSresult::DRAW),
        ("Z", &RPSresult::WIN)
    ]);

    let strategy_map = HashMap::from([
        ((&RPS::ROCK, &RPSresult::WIN), &RPS::PAPER),
        ((&RPS::PAPER, &RPSresult::WIN), &RPS::SCISSORS),
        ((&RPS::SCISSORS, &RPSresult::WIN), &RPS::ROCK),

        ((&RPS::ROCK, &RPSresult::DRAW), &RPS::ROCK),
        ((&RPS::PAPER, &RPSresult::DRAW), &RPS::PAPER),
        ((&RPS::SCISSORS, &RPSresult::DRAW), &RPS::SCISSORS),

        ((&RPS::ROCK, &RPSresult::LOSE), &RPS::SCISSORS),
        ((&RPS::PAPER, &RPSresult::LOSE), &RPS::ROCK),
        ((&RPS::SCISSORS, &RPSresult::LOSE), &RPS::PAPER),
    ]);
    
    let mut score = 0;

    for &strategy in strategies {

        // figure out what the input is
        let challenge = term_map.get(strategy.0).unwrap();
        // figure out what the expected output is
        let expected_result = result_map.get(strategy.1).unwrap();

        // figure out appropriate response
        let response = strategy_map.get(&(challenge, expected_result)).unwrap();

        score += get_score(&rps(challenge, response), response);

    }

    score
}


fn run_strategy_1(strategies: &Vec<(&str, &str)>) -> u64 {
    let term_map = HashMap::from([
        ("A", &RPS::ROCK),
        ("B", &RPS::PAPER),
        ("C", &RPS::SCISSORS),
        ("X", &RPS::ROCK),
        ("Y", &RPS::PAPER),
        ("Z", &RPS::SCISSORS)
    ]);
    let mut total = 0;
    for &strategy in strategies {

        total += apply_strategy_1(strategy, &term_map);
    }

    total
}


fn apply_strategy_1(strategy: (&str, &str), term_map: &HashMap<&str, &RPS>) -> u64 {

    let rps_input = term_map.get(strategy.0).unwrap();
    // let input_value = **rps_input as u64;
    let rps_response = term_map.get(strategy.1).unwrap();

    get_score(&rps(rps_input, rps_response), *rps_response)

}

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
enum RPS {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum RPSresult {
    WIN,
    LOSE,
    DRAW
}


fn get_score(result: &RPSresult, response: &RPS) -> u64 {

    let response_value = *response as u64;

    match result {

        RPSresult::WIN => 6 + response_value,
        RPSresult::DRAW => 3 + response_value,
        RPSresult::LOSE => 0 +  response_value,
    }
}

fn rps(challenge: &RPS, response: &RPS ) -> RPSresult {
    match (challenge, response) {
        (RPS::ROCK, RPS::PAPER) => RPSresult::WIN,
        (RPS::PAPER, RPS::SCISSORS) => RPSresult::WIN,
        (RPS::SCISSORS, RPS::ROCK) => RPSresult::WIN,
        _ if {challenge == response }=> RPSresult::DRAW,
        _ => RPSresult::LOSE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rps() {
        assert_eq!(rps(&RPS::ROCK, &RPS::PAPER), RPSresult::WIN);
        assert_eq!(rps(&RPS::PAPER, &RPS::SCISSORS), RPSresult::WIN);
        assert_eq!(rps(&RPS::SCISSORS, &RPS::ROCK), RPSresult::WIN);
        assert_eq!(rps(&RPS::ROCK, &RPS::ROCK), RPSresult::DRAW);
        assert_eq!(rps(&RPS::PAPER, &RPS::PAPER), RPSresult::DRAW);
        assert_eq!(rps(&RPS::SCISSORS, &RPS::SCISSORS), RPSresult::DRAW);
        assert_eq!(rps(&RPS::ROCK, &RPS::SCISSORS), RPSresult::LOSE);
        assert_eq!(rps(&RPS::PAPER, &RPS::ROCK), RPSresult::LOSE);
        assert_eq!(rps(&RPS::SCISSORS, &RPS::PAPER), RPSresult::LOSE);  
    }

    #[test]
    fn test_strategy() {
        
        let map = HashMap::from([
            ("A", &RPS::ROCK),
            ("B", &RPS::PAPER),
            ("C", &RPS::SCISSORS),
            ("X", &RPS::ROCK),
            ("Y", &RPS::PAPER),
            ("Z", &RPS::SCISSORS)
        ]);

        assert_eq!(apply_strategy_1(("A", "Y"), &map), 8);
        assert_eq!(apply_strategy_1(("B", "X"), &map), 1);
        assert_eq!(apply_strategy_1(("C", "Z"), &map), 6);
    }

    #[test]
    fn test_tournament() {

        let games = vec![
            ("A", "Y"),
            ("B", "X"),
            ("C", "Z")
        ];


        assert_eq!(run_strategy_1(&games), 15);
    }
}
