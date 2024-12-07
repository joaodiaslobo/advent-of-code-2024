// Day 07: Bridge Repair

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(i64, Vec<i32>)> {
    let lines : Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let mut calibration_equations = Vec::new();
    for line in lines {
        let mut iter = line.split(":").into_iter();
        let intended_result = iter.next().unwrap().parse().unwrap();
        let values = iter.next().unwrap().split(" ").filter(|x| *x != "").map(|x| {x.parse().unwrap()}).collect::<Vec<i32>>();
        calibration_equations.push((intended_result, values));
    }

    return calibration_equations;
}

#[aoc(day7, part1)]
pub fn part1(calibration_equations : &Vec<(i64, Vec<i32>)>) -> i64 {
    let mut sum_possibly_true= 0;
    for (intended_result, values) in calibration_equations {
        if check_combinations(*intended_result, values, vec!['+', '*']) {
            sum_possibly_true += *intended_result;
        }
    }

    return sum_possibly_true;
}

#[aoc(day7, part2)]
pub fn part2(calibration_equations : &Vec<(i64, Vec<i32>)>) -> i64 {
    let mut sum_possibly_true= 0;
    for (intended_result, values) in calibration_equations {
        if check_combinations(*intended_result, values, vec!['+', '*', '|']) {
            sum_possibly_true += *intended_result;
        }
    }

    return sum_possibly_true;
}

fn check_combinations(target: i64, values: &Vec<i32>, operators: Vec<char>) -> bool {
    fn dfs(index: usize, current: i64, values: &Vec<i32>, target: i64, operators: &Vec<char>) -> bool {
        if index == values.len() {
            return current == target;
        }

        let next_value = values[index] as i64;

        if operators.contains(&'+') && dfs(index + 1, current + next_value as i64, values, target, operators) {
            return true;
        }

        if operators.contains(&'*') && dfs(index + 1, current * next_value as i64, values, target, operators) {
            return true;
        }

        if operators.contains(&'|') {
            let concatenated = format!("{}{}", current, next_value).parse::<i64>().unwrap();
            if dfs(index + 1, concatenated, values, target, operators) {
                return true;
            }
        }

        false
    }

    dfs(1, values[0] as i64, values, target, &operators)
}