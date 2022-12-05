// Advent of Code Day 2
// Solved in Rust!
// Author: Logan D.G. Smith

use advent_of_rust_2022::*;


fn part_one(round: &String) -> u32 {
    let choices: Vec<char> = round.chars().collect();
    match choices[0] {
        'A' => {
            match choices [2] {
                'X' => 4, // Rock v. Rock
                'Y' => 8, // Rock v. Paper
                'Z' => 3, // Rock v. Scissors
                _ => { panic!("Got char '{}'", choices[2]); }
            }
        },
        'B' => {
            match choices [2] {
                'X' => 1, // Paper v. Rock
                'Y' => 5, // Paper v. Paper
                'Z' => 9, // Paper v. Scissors
                _ => { panic!("Got char '{}'", choices[2]); }
            }
        },
        'C' => {
            match choices [2] {
                'X' => 7, // Scissors v. Rock
                'Y' => 2, // Scissors v. Paper
                'Z' => 6, // Scissors v. Scissors
                _ => { panic!("Got char '{}'", choices[2]); }
            }
        }
        _ => {
            panic!("Got char '{}'", choices[0]);
        }
    }
}

fn part_two(round: &String) -> u32 {
    let choices: Vec<char> = round.chars().collect();
    match choices[0] {
        'A' => {
            match choices [2] {
                'X' => 3, // Rock v. Scissors
                'Y' => 4, // Rock v. Rock
                'Z' => 8, // Rock v. Paper
                _ => { panic!("Got char '{}'", choices[2]); }
            }
        },
        'B' => {
            match choices [2] {
                'X' => 1, // Paper v. Rock
                'Y' => 5, // Paper v. Paper
                'Z' => 9, // Paper v. Scissors
                _ => { panic!("Got char '{}'", choices[2]); }
            }
        },
        'C' => {
            match choices [2] {
                'X' => 2, // Scissors v. Paper
                'Y' => 6, // Scissors v. Scissors
                'Z' => 7, // Scissors v. Rock
                _ => { panic!("Got char '{}'", choices[2]); }
            }
        }
        _ => {
            panic!("Got char '{}'", choices[0]);
        }
    }
}

fn main() {
    let mut part_one_score: u32 = 0;
    let mut part_two_score: u32 = 0;

    let lines = read_lines("inputs/day2.txt").expect("Failed to read file!");

    for line in lines {
        if let Ok(entry) = line {
            let part_one_round: u32 = part_one(&entry);
            let part_two_round: u32 = part_two(&entry);

            part_one_score += part_one_round;
            part_two_score += part_two_round;
        }
    }

    println!("Part 1 Score: {}", part_one_score);
    println!("Part 2 Score: {}", part_two_score);
}
