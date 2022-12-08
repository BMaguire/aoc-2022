extern crate common;


use common::file_util;

use std::collections::HashMap;

fn main() {
    println!("Advent of Code Day 8");
    let data = file_util::read_file("../inputs/day_8.txt", "\n");

    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<String>) {
    println!("Part 1");

    // calculate the number of times a depth measurement increases
    println!("Answer {:?}", scan_forest(process_input(&data)));
}

fn part_2(data: &Vec<String>) {
    println!("Part 2");
    println!("Answer {:?}", find_max_scenic_score(process_input(&data)));
}

fn process_input(data: &Vec<String>) -> (Forest, u64, u64) {
    let mut forest : HashMap<(u64, u64),u64> = HashMap::new();
    let mut width: u64 = 0;
    let mut height: u64 = 0;
    for (row, line) in data.iter().enumerate() {
        width += 1;

        for (col, character) in line.split("").filter(|a| *a != "").collect::<Vec<&str>>().iter().enumerate() {
            if height <= col as u64 {
                height += 1
            }
            forest.insert((row as u64,col as u64), character.parse::<u64>().unwrap());

        }
        
    }
    (forest, width, height)
}

fn find_max_scenic_score((forest_map, width, height):(Forest, u64, u64)) -> u64 {
    let mut current_max = 0;
    let forest = &forest_map;
    for tree in (0..height).map(|row| (0..width).map(move |col| (row,col))).flatten() {
        // println!("{:?}", tree);

        let score = calculate_scenic_score(tree, forest);
        if score > current_max { current_max = score }
    }

    current_max
}

type Forest = HashMap<(u64, u64),u64>;

fn scan_forest((forest_map, width, height):(Forest, u64, u64)) -> u64 {
    let forest = &forest_map;

    let mut spotted_trees = HashMap::new();

    // left
    for trees in (0..height).map(|row| (0..width).map(move |col| (row, col, forest.get(&(row,col)).unwrap()))) {
        for spotted_tree in scan_treeline(trees) {
            spotted_trees.insert(spotted_tree,1);
        };
    }

    // right
    for trees in (0..height).map(|row| (0..width).map( move|col| (row, col, forest.get(&(row,col)).unwrap()))){
        for spotted_tree in scan_treeline(trees.rev()) {
            spotted_trees.insert(spotted_tree,1);
        };
    }

    // top
    for trees in (0..width).map(|col| (0..height).map( move|row| (row, col, forest.get(&(row,col)).unwrap()))){
        for spotted_tree in scan_treeline(trees) {
            spotted_trees.insert(spotted_tree,1);
        };
    }

    // bottom
    for trees in (0..width).map(|col| (0..height).map( move|row| (row, col, forest.get(&(row,col)).unwrap()))){
        for spotted_tree in scan_treeline(trees.rev()) {
            spotted_trees.insert(spotted_tree,1);
        };
    }


    spotted_trees.keys().len() as u64
}

// returns visible trees
fn scan_treeline<'a, I>(trees: I ) -> Vec<(u64,u64)>
    where I: Iterator<Item = (u64, u64, &'a u64)> {
    let mut current_top = -1;
    let mut visible_trees: Vec<(u64,u64)> = vec![]; 
    for ( row, col, &tree) in trees {
        if tree as i64 > current_top {

            current_top = tree as i64;
            visible_trees.push((row, col));
        }

        if tree == 9 { break };
    }



    visible_trees
}

fn calculate_scenic_score(candidate: (u64, u64), forest: &Forest) -> u64 { 

    let candidate_tree = forest.get(&candidate).unwrap();

    // look north
    let north_trees = project_view(forest, candidate, (-1,0), *candidate_tree);
    // look south
    let south_trees = project_view(forest, candidate, (1,0), *candidate_tree);    
    // look east
    let east_trees = project_view(forest, candidate, (0,-1), *candidate_tree);
    // look west
    let west_trees = project_view(forest, candidate, (0,1), *candidate_tree);

    north_trees * south_trees * east_trees * west_trees
}

fn project_view(forest: &Forest, start: (u64, u64), direction: (i64,i64), threshold: u64) -> u64 {
    let mut cursor = start;
    let mut count = 0;

    cursor.0 = (cursor.0 as i64 + direction.0) as u64;
    cursor.1 = (cursor.1 as i64 + direction.1) as u64;

    while let Some(&tree) = forest.get(&cursor) {
        count +=1;
        cursor.0 = (cursor.0 as i64 + direction.0) as u64;
        cursor.1 = (cursor.1 as i64 + direction.1) as u64;

        if tree >= threshold { break }
    }
    
    count
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_scan_forest() {
        let input: HashMap<(u64, u64),u64> = HashMap::from([
            ((0,0), 3),
            ((0,1), 0),
            ((0,2), 3),
            ((0,3), 7),
            ((0,4), 3),

            ((1,0), 2),
            ((1,1), 5),
            ((1,2), 5),
            ((1,3), 1),
            ((1,4), 2),

            ((2,0), 6),
            ((2,1), 5),
            ((2,2), 3),
            ((2,3), 3),
            ((2,4), 2),
            
            ((3,0), 3),
            ((3,1), 3),
            ((3,2), 5),
            ((3,3), 4),
            ((3,4), 9),


            ((4,0), 3),
            ((4,1), 5),
            ((4,2), 3),
            ((4,3), 9),
            ((4,4), 0),
        ]);

        assert_eq!(scan_forest((input, 5, 5)), 21);
    }

    #[test]
    fn test_scenic_score() {

        let input: HashMap<(u64, u64),u64> = HashMap::from([
            ((0,0), 3),
            ((0,1), 0),
            ((0,2), 3),
            ((0,3), 7),
            ((0,4), 3),

            ((1,0), 2),
            ((1,1), 5),
            ((1,2), 5),
            ((1,3), 1),
            ((1,4), 2),

            ((2,0), 6),
            ((2,1), 5),
            ((2,2), 3),
            ((2,3), 3),
            ((2,4), 2),
            
            ((3,0), 3),
            ((3,1), 3),
            ((3,2), 5),
            ((3,3), 4),
            ((3,4), 9),


            ((4,0), 3),
            ((4,1), 5),
            ((4,2), 3),
            ((4,3), 9),
            ((4,4), 0),
        ]);

        assert_eq!(calculate_scenic_score((3,2), &input), 8);
    }
}
