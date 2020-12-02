mod days;
mod misc;
fn main() {
    days::day01::run();
    days::day01::bench(&mut Vec::new());
    days::day02::run().unwrap();
}
