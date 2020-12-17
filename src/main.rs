#![allow(dead_code)]

use std::time::Instant;

mod computer;
mod days;
mod misc;
fn main() {
    let start = Instant::now();
    days::day17::run();
    println!("{}", start.elapsed().as_secs());
    //start();
}
fn start() {
    days::day01::run();
    //days::day01::bench(&mut Vec::new());
    days::day02::run().unwrap();
    days::day03::run().unwrap();
    days::day04::run().unwrap();
    days::day05::run().unwrap();
    days::day06::run().unwrap();
    days::day07::run().unwrap();
    days::day08::run().unwrap();
    days::day09::run();
    days::day10::run();
    days::day11::run();
    days::day12::run();
    days::day13::run();
    days::day14::run();
    days::day15::run();
    days::day16::run();
}
