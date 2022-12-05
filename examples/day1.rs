// Advent of Code Day 1
// Solved in Rust!
// Author: Logan D.G. Smith
// Don't mind part one, I was still warming up.

use std::collections::BinaryHeap;

use advent_of_rust_2022::*;

fn part_one() {
    let mut current_calories: i32 = 0;
    let mut current_elf: i32 = 1;

    let mut most_calories: (i32, i32) = (0, 0);

    let lines = read_lines("inputs/day1.txt").expect("Failed to read file!");

    for line in lines {
        if let Ok(entry) = line {
            if entry == "" {
                if current_calories > most_calories.1 {
                    // println!("New High Cal! Elf: {}, Calories: {}", current_elf, current_calories);
                    most_calories = (current_elf, current_calories);
                }

                current_calories = 0;
                current_elf += 1;
            }
            else {
                current_calories += entry.parse::<i32>().unwrap();
            }
        }
    }

    println!("Elf with most calories: {}, with {} calories", most_calories.0, most_calories.1);

}

fn part_two() {
    let mut rankings_heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut current_calories: i32 = 0;

    let lines = read_lines("inputs/day1.txt").expect("Failed to read file!");

    for line in lines {
        if let Ok(entry) = line {
            if entry == "" {
                rankings_heap.push(current_calories);
                current_calories = 0;
            }
            else {
                current_calories += entry.parse::<i32>().expect("Failed to parse line!");
            }
        }
    }

    let mut top_total: i32 = 0;
    for _ in 0..3 {
        let ranking: i32 = rankings_heap.pop().expect("Failed to pop heap!");
        println!("This elf is carrying {} calories", ranking);

        top_total += ranking;
    }

    println!("Top Three Elves are carrying {} calories", top_total);
}

fn main() {
    part_one();
    println!("---");
    part_two();
}
