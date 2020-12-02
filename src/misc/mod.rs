use crate::misc::error::{AoCError, AoCResult};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod error;
pub fn read_vec_ints(filename: String) -> error::AoCResult<Vec<usize>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    for line in reader.lines() {
        data.push(line?.parse()?);
    }
    Ok(data)
}
pub fn read_vec_string(filename: String) -> error::AoCResult<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    for line in reader.lines() {
        data.push(line?);
    }
    Ok(data)
}
/// Attempts to retrieve a value from a vec, and converts to an AoC Error if it fails
pub fn get_values<T: std::fmt::Debug>(data: &[T], index: usize) -> AoCResult<&T> {
    data.get(index).ok_or_else(|| {
        AoCError::new(format!(
            "Couldn't index vector!\nIndex: {}\nData: {:?}",
            index, data,
        ))
    })
}
