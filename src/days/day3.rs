// Day 3: Mull It Over

use crate::utils;
use regex::Regex;

pub fn first_answer() {
    let mut mul_sum = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Read lines from input
    if let Ok(lines) = utils::read_lines("input/3.txt") {    
        for line in lines.flatten() {
            for cap in re.captures_iter(&line) {
                // Extract the two numbers using capture groups
                let num1: i32 = cap[1].parse().unwrap();
                let num2: i32 = cap[2].parse().unwrap();

                // Multiply and add to the sum
                mul_sum += num1 * num2;
            }
        }
    }

    println!("First challenge solution: {}.", mul_sum);
}

pub fn second_answer() {
    let mut mul_sum = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    // Read lines from input
    if let Ok(lines) = utils::read_lines("input/3.txt") {
        let mut can_mul = true;
        for line in lines.flatten() {
            for cap in re.captures_iter(&line) {
                if let Some(mul_match) = cap.get(0) {
                    let matched_text = mul_match.as_str();

                    if matched_text.starts_with("mul") && can_mul {
                        // Extract the two numbers for mul
                        let num1: i32 = cap[1].parse().unwrap();
                        let num2: i32 = cap[2].parse().unwrap();
                        mul_sum += num1 * num2;
                    } else if matched_text == "do()" {
                        can_mul = true;
                    } else if matched_text == "don't()" {
                        can_mul = false;
                    }
                }
            }
        }
    }

    println!("Second challenge solution: {}.", mul_sum);
}