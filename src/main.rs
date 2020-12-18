#![allow(dead_code)]

use std::time::Instant;

mod computer;
mod days;
mod misc;

fn main() {
    let start_time = Instant::now();
    //days::day18::run();
    //println!("    Time: {}ms", start_time.elapsed().as_secs());
    run_all();
    println!("Total Time: {}ms", start_time.elapsed().as_millis());
}
fn run_all() {
    println!("\nDay 1:");
    let mut start = Instant::now();
    days::day01::run();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 2:");
    start = Instant::now();
    days::day02::run().unwrap();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 3:");
    start = Instant::now();
    days::day03::run().unwrap();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 4:");
    start = Instant::now();
    days::day04::run().unwrap();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 5:");
    start = Instant::now();
    days::day05::run().unwrap();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 6:");
    start = Instant::now();
    days::day06::run().unwrap();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 7:");
    start = Instant::now();
    days::day07::run().unwrap();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 8:");
    start = Instant::now();
    days::day08::run().unwrap();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 9:");
    start = Instant::now();
    days::day09::run();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 10:");
    start = Instant::now();
    days::day10::run();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 11:");
    start = Instant::now();
    days::day11::run();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 12:");
    start = Instant::now();
    days::day12::run();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 13:");
    start = Instant::now();
    days::day13::run();
    println!("  Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 14:");
    start = Instant::now();
    days::day14::run();
    println!("  Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 15:");
    start = Instant::now();
    days::day15::run();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 16:");
    start = Instant::now();
    days::day16::run();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 17:");
    start = Instant::now();
    days::day17::run();
    println!("    Time: {}ms", start.elapsed().as_millis());

    println!("\nDay 18:");
    start = Instant::now();
    days::day18::run();
    println!("    Time: {}ms", start.elapsed().as_millis());
}
