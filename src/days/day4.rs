// Day 4: Ceres Search

use crate::utils;

pub fn first_answer() {
    let mut xmas_count = 0;

    if let Ok(lines) = utils::read_lines("input/4.txt") {
        let char_grid: Vec<Vec<char>> = lines
            .filter_map(Result::ok)
            .map(|line| line.chars().collect())
            .collect();

        for (y, row) in char_grid.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                if ch == 'X' {
                    xmas_count += check_all_directions(&char_grid, x as i32, y as i32);
                }
            }
        }
    }

    println!("First challenge solution: {}.", xmas_count);
}

pub fn second_answer() {
    let mut x_mas_count = 0;

    if let Ok(lines) = utils::read_lines("input/4.txt") {
        let char_grid: Vec<Vec<char>> = lines
            .filter_map(Result::ok)
            .map(|line| line.chars().collect())
            .collect();

        for (y, row) in char_grid.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                if ch == 'A' &&  is_mas_cross(&char_grid, x as i32, y as i32)  {
                    x_mas_count += 1;
                }
            }
        }
    }

    println!("Second challenge solution: {}.", x_mas_count);
}

fn check_all_directions(grid: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let directions = [
        (1, 0),  
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, -1),
        (-1, -1),
        (1, 1),
        (-1, 1),
    ];

    let mut count = 0;
    for &(dx, dy) in &directions {
        if check_direction(grid, x, y, dx, dy) {
            count += 1;
        }
    }
    count
}

fn check_direction(grid: &Vec<Vec<char>>, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    let sequence = ['M', 'A', 'S'];
    for (i, &expected_char) in sequence.iter().enumerate() {
        if get_char(grid, x + dx * (i as i32 + 1), y + dy * (i as i32 + 1)) != expected_char {
            return false;
        }
    }
    true
}

fn get_char(grid: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    if y < 0 || y >= grid.len() as i32 || x < 0 || x >= grid[0].len() as i32 {
        '\0'
    } else {
        grid[y as usize][x as usize]
    }
}

fn is_mas_cross(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    (
        (get_char(grid, x - 1, y - 1) == 'M' && get_char(grid, x + 1, y + 1) == 'S') 
            || (get_char(grid, x - 1, y - 1) == 'S' && get_char(grid, x + 1, y + 1) == 'M')
        )
    &&  (
        (get_char(grid, x + 1, y - 1) == 'M' && get_char(grid, x - 1, y + 1) == 'S') 
            || (get_char(grid, x + 1, y - 1) == 'S' && get_char(grid, x - 1, y + 1) == 'M')
    )
}