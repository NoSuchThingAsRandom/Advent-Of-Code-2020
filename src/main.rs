#![allow(dead_code)]
mod computer;
mod days;
mod misc;
fn main() {
    days::day13::run();
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
}
