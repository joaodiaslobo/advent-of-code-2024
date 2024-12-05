// Day 1: Historian Hysteria

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day1, part1)]
pub fn part1(lines : &Vec<String>) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        // Split each line, and store left and right values in the specific list
        let location_ids = line.split("   ").collect::<Vec<&str>>();
        left.push(location_ids[0].parse().expect("Failed parse."));
        right.push(location_ids[1].parse().expect("Failed parse"));
    }

    // Sort the lists
    left.sort();
    right.sort();

    let mut total_distance: i32 = 0;

    // For each element in left list, find and sum to total the distance from element in the same index in right list
    for (i, position_id) in left.iter().enumerate() {
        total_distance += (position_id - right[i]).abs();
    }

    total_distance
}

#[aoc(day1, part2)]
pub fn part2(lines : &Vec<String>) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        // Split each line, and store left and right values in the specific list
        let location_ids = line.split("   ").collect::<Vec<&str>>();
        left.push(location_ids[0].parse().expect("Failed parse."));
        right.push(location_ids[1].parse().expect("Failed parse"));
    }

    // Sort the left list
    left.sort();

    let mut similarity_score: i32 = 0;

    // For each element in left list, find occurences of element in right list and calculate the similarity score
    for position_id in left {
        let count = right.iter().filter(|&n| *n == position_id).count() as i32;
        similarity_score += position_id * count;
    }

    similarity_score
}