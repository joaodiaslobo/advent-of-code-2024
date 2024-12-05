// Day 5: Print Queue

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let lines : Vec<String> = input.lines().map(|l| l.to_string()).collect();
    
    let mut is_pages = false;

    let mut pages = Vec::new();
    let mut rules = Vec::new();
    
    for line in lines {
        if line == "" {
            is_pages = true; 
        } else if is_pages {
            pages.push(line.split(",").map(|x| x.parse().unwrap()).collect());
        } else {
            let mut iter = line.split("|").map(|x| x.parse().unwrap()).into_iter();
            rules.push((iter.next().unwrap(), iter.next().unwrap()));
        }
    }

    (rules, pages)
}

#[aoc(day5, part1)]
pub fn part1((rules, lists) : &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let mut valid_mid_sum = 0;
    for pages in lists {
        let mut valid = true;
        for (i, _page) in pages.iter().enumerate() {
            // Check if page positioning respects all rules
            if !page_respects_rules(i, pages, rules) {
                valid = false;
                break;
            }
        }

        if valid {
            valid_mid_sum += pages[pages.len() / 2];
        }
    }

    valid_mid_sum
}

#[aoc(day5, part2)]
pub fn part2((rules, lists) : &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let mut mid_sum = 0;
    for pages in lists {
        for (i, _page) in pages.iter().enumerate() {
            // Check if page positioning respects all rules
            if !page_respects_rules(i, pages, rules) {
                let sorted = sort_until_respects_rules(pages, rules);
                mid_sum += sorted[sorted.len() / 2];
                break;
            }
        }
    }

    mid_sum
}

fn page_respects_rules(index : usize, pages : &Vec<i32>, rules : &Vec<(i32, i32)>) -> bool {
    let page_number = pages[index];

    for (before, after) in rules {
        if *before == page_number {
            for i in 0..index {
                if pages[i] == *after {
                    return false;
                }
            }
        } else if *after == page_number {
            for i in index+1..pages.len() {
                if pages[i] == *before {
                    return false;
                }
            }

        }
    }

    true
}

fn sort_until_respects_rules(pages : &Vec<i32>, rules : &Vec<(i32, i32)>) -> Vec<i32> {
    let mut sorted = pages.clone();
    let mut sorted_respects_rules = false;
    while !sorted_respects_rules {
        sorted_respects_rules = true;
        for i in 0..sorted.len() - 1 {
            if !page_respects_rules(i, &sorted, rules) {
                sorted_respects_rules = false;
                sorted.swap(i, i+1);
            }
        }
    }

    sorted
}