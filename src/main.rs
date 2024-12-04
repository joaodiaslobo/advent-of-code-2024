use std::io;

mod days;
mod utils;

fn main() {
    println!("== 🎄  ADVENT OF CODE 2024 🎄 ==");
    println!("Select a day:");

    // Get day input from user
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Couldn't read line.");

    // Parse the input safely
    let selected_day: i32 = match input_line.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter an integer.");
            return;
        }
    };

    // Run solutions for chosen day
    match selected_day {
        1 => {
            days::day1::fist_answer();
            days::day1::second_answer();
        }
        2 => {
            days::day2::fist_answer();
            days::day2::second_answer();
        }
        3 => {
            days::day3::first_answer();
            days::day3::second_answer();
        }
        4 => {
            days::day4::first_answer();
            days::day4::second_answer();
        }
        _ => println!("No solution for this day.")
    }
}
