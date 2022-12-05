extern crate common;


// use itertools::Itertools;
use common::file_util;

fn main() {
    println!("Advent of Code Day 1");
    let data = file_util::read_file("../inputs/day_1.txt", "\n");

    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<String>) {
    println!("Part 1");

    // calculate the number of times a depth measurement increases
    println!("Answer {:?}", count_callories(&data));
}

fn part_2(data: &Vec<String>) {
    println!("Part 2");
    println!("Answer {:?}", count_top_three(&data));
}

fn count_callories(calorie_entries: &Vec<String>) -> u64 {
    let mut max = 0;
    let mut count = 0;
    for entry in calorie_entries.iter() {
        match entry.as_str() {
            "" => {
                // if current count is larger than max
                if count > max {
                    max = count
                }
                // reset count
                count = 0
            },
            _ => {
                // append to the count
                count += entry.parse::<u64>().unwrap();
            }
        }
    }   

    max
}

fn count_top_three(calorie_entries: &Vec<String>) -> u64 {
//     depths.iter().tuple_windows::<(_,_,_)>().map(|(a,b,c)| a+b+c ).tuple_windows().map(|(a,b)| { if a < b { 1 }  else { 0 } }).sum()
    let mut max_set: Vec<u64> = vec![];
    let mut count = 0;
    for entry in calorie_entries.iter() {
        match entry.as_str() {
            "" => {
                // append to set of max_sets
                max_set.push(count);    
                // reset count
                count = 0
            },
            _ => {
                // append to the count
                count += entry.parse::<u64>().unwrap();
            }
        }
    }
    // sort max set and sum top 3 values
    max_set.sort();
    max_set[max_set.len()-3..max_set.len()].iter().sum::<u64>()
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = vec!["1000","2000","3000",""].into_iter().map(|a| a.to_string()).collect();
        assert_eq!(count_callories(&input), 6000);
    }
}
