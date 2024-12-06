// Day 6: Guard Gallivant

use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Clone)]
struct Square {
    visited : bool,
    obstacle: bool   
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT
}

#[derive(Clone)]
pub struct Map {
    guard: Guard,
    squares: Vec<Vec<Square>>
}

#[derive(Clone)]
struct Guard {
    position: (i32, i32),
    direction: Direction
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Map {
    let lines : Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let mut squares = vec![vec![Square{visited: false, obstacle: true}; lines[0].len()]; lines.len()];
    let mut guard = Guard{position: (0, 0), direction: Direction::UP};
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' {
                let sq = Square{visited: false, obstacle: false};
                squares[y][x] = sq;
            } else if ch == '#' {
                let sq = Square{visited: false, obstacle: true};
                squares[y][x] = sq;
            } else if ch == '^' {
                let sq = Square{visited: false, obstacle: false};
                squares[y][x] = sq;
                guard.position = (x as i32, y as i32);
            }
        }
    }

    return  Map{guard: guard, squares: squares};
}

#[aoc(day6, part1)]
pub fn part1(original_map: &Map) -> i32 {
    let mut map: Map = original_map.clone();
    let mut distinct_positions = 0;

    while map.guard.position.0 >= 0
        && map.guard.position.0 < map.squares[0].len() as i32
        && map.guard.position.1 >= 0
        && map.guard.position.1 < map.squares.len() as i32
    {
        // Current square
        let square = &mut map.squares[map.guard.position.1 as usize][map.guard.position.0 as usize];

        // Count distinct positions
        if !square.visited {
            square.visited = true;
            distinct_positions += 1;
        }

        // Calculate the next position
        let next_position = next_position(map.guard.position, map.guard.direction);

        // Move or change direction
        if can_move_to(&map, next_position) {
            map.guard.position = next_position;
        } else {
            map.guard.direction = rotate_90_degrees(map.guard.direction);
        }
    }

    distinct_positions
}

#[aoc(day6, part2)]
pub fn part2(original_map: &Map) -> i32 {
    let map: Map = original_map.clone();
    let mut possible_obstructions = 0;

    let map_height = map.squares.len() as i32;
    let map_width = map.squares[0].len() as i32;

    for y in 0..map_height {
        for x in 0..map_width {
            if !map.squares[y as usize][x as usize].obstacle && map.guard.position != (x, y) {
                if test_obstruction(&map, (x, y)) {
                    possible_obstructions += 1;
                }
            }
        }
    }

    possible_obstructions
}

fn test_obstruction(original_map: &Map, obstruction_position: (i32, i32)) -> bool {
    let mut map = original_map.clone();

    // Place the obstruction
    map.squares[obstruction_position.1 as usize][obstruction_position.0 as usize].obstacle = true;

    let mut states: HashSet<(i32, i32, Direction)> = HashSet::new();

    while map.guard.position.0 >= 0
        && map.guard.position.0 < map.squares[0].len() as i32
        && map.guard.position.1 >= 0
        && map.guard.position.1 < map.squares.len() as i32
    {
        // Check if the guard is stuck in a loop
        if !states.insert((map.guard.position.0, map.guard.position.1, map.guard.direction)) {
            return true;
        }

        // Calculate the next position
        let next_position = next_position(map.guard.position, map.guard.direction);

        // Move or change direction
        if can_move_to(&map, next_position) {
            map.guard.position = next_position;
        } else {
            map.guard.direction = rotate_90_degrees(map.guard.direction);
        }
    }

    false
}


fn can_move_to(map : &Map, position : (i32, i32)) -> bool {
    if position.0 < 0 || position.0 >= map.squares[0].len() as i32 || position.1 < 0 || position.1 >= map.squares.len() as i32 {
        return true;
    }

    return !map.squares[position.1 as usize][position.0 as usize].obstacle;
}

fn next_position(position : (i32, i32), direction : Direction) -> (i32, i32) {
    match direction {
        Direction::UP => (position.0, position.1 - 1),
        Direction::DOWN => (position.0, position.1 + 1),
        Direction::RIGHT => (position.0 + 1, position.1),
        Direction::LEFT => (position.0 - 1, position.1)
    }
}

fn rotate_90_degrees(direction : Direction) -> Direction {
    match direction {
        Direction::UP => Direction::RIGHT,
        Direction::DOWN => Direction::LEFT,
        Direction::RIGHT => Direction::DOWN,
        Direction::LEFT => Direction::UP
    }
}