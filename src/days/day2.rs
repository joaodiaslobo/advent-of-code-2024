// Day 2: Red-Nosed Reports

use crate::utils;

pub fn fist_answer() {
    let mut safe_count = 0;

    // Read lines from input
    if let Ok(lines) = utils::read_lines("input/2.txt") {
        for line in lines.flatten() {
            // Split each line, and store left and right values in the specific list
            let levels: Vec<i32> = line
                .split(" ")
                .filter_map(|n_str| n_str.parse::<i32>().ok())
                .collect();

            if !are_levels_unsafe(levels) {
                safe_count += 1;    
            }    
        }
    }

    println!("First challenge solution: {}.", safe_count);
}

pub fn second_answer() {
    let mut safe_count = 0;

    // Read lines from input
    if let Ok(lines) = utils::read_lines("input/2.txt") {
        for line in lines.flatten() {
            // Split each line, and store left and right values in the specific list
            let levels: Vec<i32> = line
                .split(" ")
                .filter_map(|n_str| n_str.parse::<i32>().ok())
                .collect();

            // If levels are unsafe, test removing level by level
            if are_levels_unsafe(levels.clone()) {
                for (i, _level) in levels.iter().enumerate() {
                    let mut new_levels = levels.clone();
                    new_levels.remove(i);
                    if !are_levels_unsafe(new_levels){
                        safe_count += 1;
                        break;
                    }
                }  
            }  else {
                safe_count += 1;
            }  
        }
    }

    println!("Second challenge solution: {}.", safe_count);
}

// Checks all conditions
fn are_levels_unsafe(levels : Vec<i32>) -> bool {
    let mut is_unsafe = false;
    let mut sign : Option<i32> = None;

    for (i, level) in levels.iter().enumerate() {
        if i == levels.len() - 1 { break; }

        let level_difference = level - levels[i + 1]; 

        if sign.is_none() { sign = Some(level_difference.signum()) }

        if level_difference.abs() < 1 || level_difference.abs() > 3 || level_difference.signum() != sign.expect("not initialized") {
            is_unsafe = true;
            break;
        }
    }

    return is_unsafe;
}