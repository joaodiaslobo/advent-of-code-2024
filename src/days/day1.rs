// Day 1: Historian Hysteria

use crate::utils;

pub fn fist_answer() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // Read lines from input
    if let Ok(lines) = utils::read_lines("input/1.txt") {
        for line in lines.flatten() {
            // Split each line, and store left and right values in the specific list
            let location_ids = line.split("   ").collect::<Vec<&str>>();
            left.push(location_ids[0].parse().expect("Failed parse."));
            right.push(location_ids[1].parse().expect("Failed parse"));
        }
    }

    // Sort the lists
    left.sort();
    right.sort();

    let mut total_distance: i32 = 0;

    // For each element in left list, find and sum to total the distance from element in the same index in right list
    for (i, position_id) in left.iter().enumerate() {
        total_distance += (position_id - right[i]).abs();
    }

    println!("First challenge solution: {}.", total_distance);
}

pub fn second_answer() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // Read lines from input
    if let Ok(lines) = utils::read_lines("input/1.txt") {
        for line in lines.flatten() {
            // Split each line, and store left and right values in the specific list
            let location_ids = line.split("   ").collect::<Vec<&str>>();
            left.push(location_ids[0].parse().expect("Failed parse."));
            right.push(location_ids[1].parse().expect("Failed parse"));
        }
    }

    // Sort the left list
    left.sort();

    let mut similarity_score: i32 = 0;

    // For each element in left list, find occurences of element in right list and calculate the similarity score
    for position_id in left {
        let count = right.iter().filter(|&n| *n == position_id).count() as i32;
        similarity_score += position_id * count;
    }

    println!("Second challenge solution: {}.", similarity_score);
}