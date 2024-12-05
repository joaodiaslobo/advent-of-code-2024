// Day 3: Mull It Over

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn part1(lines : &Vec<String>) -> i32 {
    let mut mul_sum = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for line in lines {
        for cap in re.captures_iter(&line) {
            // Extract the two numbers using capture groups
            let num1: i32 = cap[1].parse().unwrap();
            let num2: i32 = cap[2].parse().unwrap();

            // Multiply and add to the sum
            mul_sum += num1 * num2;
        }
    }

    mul_sum
}

#[aoc(day3, part2)]
pub fn part2(lines : &Vec<String>) -> i32 {
    let mut mul_sum = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut can_mul = true;
    for line in lines {
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

    mul_sum
}