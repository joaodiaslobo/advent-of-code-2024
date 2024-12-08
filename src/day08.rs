// Day 08: Resonant Collinearity

use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
struct Square {
    square_type: char
}

#[derive(Clone)]
pub struct Map {
    squares: Vec<Vec<Square>>
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Map {
    let lines : Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let mut squares = vec![vec![Square{square_type: '.'}; lines[0].len()]; lines.len()];

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            squares[y][x].square_type = ch;
        }
    }

    return Map{squares: squares};
}

#[aoc(day8, part1)]
pub fn part1(map: &Map) -> usize {
    let mut antinodes = HashSet::new();
    let height = map.squares.len();
    let width = map.squares[0].len();

    let mut antennas_by_frequency: std::collections::HashMap<char, Vec<(usize, usize)>> =
        std::collections::HashMap::new();

    for (y, row) in map.squares.iter().enumerate() {
        for (x, square) in row.iter().enumerate() {
            if square.square_type != '.' {
                antennas_by_frequency
                    .entry(square.square_type)
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
        }
    }

    for (_, antennas) in antennas_by_frequency {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let (x1, y1) = antennas[i];
                let (x2, y2) = antennas[j];

                let xd = x2 as isize - x1 as isize;
                let yd = y2 as isize - y1 as isize;

                let antinode1 = (
                    x1 as isize - xd,
                    y1 as isize - yd,
                );

                let antinode2 = (
                    x2 as isize + xd,
                    y2 as isize + yd,
                );

                if antinode1.0 >= 0
                    && antinode1.0 < width as isize
                    && antinode1.1 >= 0
                    && antinode1.1 < height as isize
                {
                    antinodes.insert((antinode1.0 as usize, antinode1.1 as usize));
                }

                if antinode2.0 >= 0
                    && antinode2.0 < width as isize
                    && antinode2.1 >= 0
                    && antinode2.1 < height as isize
                {
                    antinodes.insert((antinode2.0 as usize, antinode2.1 as usize));
                }
            }
        }
    }

    antinodes.len()
}

#[aoc(day8, part2)]
pub fn part2(map: &Map) -> usize {
    let mut antinodes = HashSet::new();
    let height = map.squares.len();
    let width = map.squares[0].len();

    let mut antennas_by_frequency: std::collections::HashMap<char, Vec<(usize, usize)>> =
        std::collections::HashMap::new();

    for (y, row) in map.squares.iter().enumerate() {
        for (x, square) in row.iter().enumerate() {
            if square.square_type != '.' {
                antennas_by_frequency
                    .entry(square.square_type)
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
        }
    }

    for (_, antennas) in antennas_by_frequency {

        for &(x, y) in &antennas {
            antinodes.insert((x, y));
        }

        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let (x1, y1) = antennas[i];
                let (x2, y2) = antennas[j];

                let xd = x2 as isize - x1 as isize;
                let yd = y2 as isize - y1 as isize;
                let mut mul = 1 as isize;

                loop {
                    let antinode1 = (
                        x1 as isize - xd * mul,
                        y1 as isize - yd * mul,
                    );

                    let antinode2 = (
                        x2 as isize + xd * mul,
                        y2 as isize + yd * mul,
                    );

                    let mut both_failed = true;

                    if antinode1.0 >= 0
                        && antinode1.0 < width as isize
                        && antinode1.1 >= 0
                        && antinode1.1 < height as isize
                    {
                        both_failed = false;
                        antinodes.insert((antinode1.0 as usize, antinode1.1 as usize));
                    }

                    if antinode2.0 >= 0
                        && antinode2.0 < width as isize
                        && antinode2.1 >= 0
                        && antinode2.1 < height as isize
                    {
                        both_failed = false;
                        antinodes.insert((antinode2.0 as usize, antinode2.1 as usize));
                    }


                    if both_failed {
                        break;
                    }

                    mul += 1;
                }
            }
        }
    }

    antinodes.len()
}