use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod error;
pub fn read_vec(filename: String) -> error::AoCResult<Vec<usize>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    for line in reader.lines() {
        data.push(line?.parse()?);
    }
    Ok(data)
}
