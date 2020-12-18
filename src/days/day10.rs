use crate::misc::error::{AoCError, AoCResult};
use crate::misc::read_vec_ints;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let mut data = read_vec_ints(String::from("Inputs/test10a.txt")).unwrap();
    data.sort();
    //data.sort();
    println!("    Max volt rating: {}", part_1(data).unwrap());
    //println!("Iterations: {}", part_2(&data, 0, 1, 22));
    //println!("Iterations: {:?}", find_critical_path(data).unwrap());
    //println!("Possibile: {:?}", part_2c(data).unwrap());
    //println!("Iterations: {}", part_2_a(&data));
}

fn part_1(mut volts: Vec<usize>) -> AoCResult<usize> {
    // For the final adapter having a volt dif of 3
    let mut volt_differences: [usize; 3] = [0, 0, 1];
    volts.sort();
    let mut current_max_volts = 0;
    for adapter in volts {
        let dif = adapter - current_max_volts;
        if 0 < dif && dif <= 3 {
            volt_differences[dif - 1] += 1;
            current_max_volts = adapter;
        } else {
            break;
        }
    }
    //println!("Highest volt: {}", current_max_volts + 3);
    Ok(volt_differences[0] * volt_differences[2])
}

// Has to be sorted data
fn find_critical_path_reversed(mut volts: Vec<usize>) -> AoCResult<Vec<usize>> {
    //volts.push(0);
    volts.sort();
    volts.reverse();
    let mut index = 0;
    let mut temp_index = 0;

    let mut current = volts[0];

    let mut path = Vec::new();
    path.push(current);

    while current != 0 {
        for x in index..volts.len() {
            let dif = volts[index] - volts[x];
            if 0 < dif && dif <= 3 {
                //println!("Current: {}, X_Volt {}, Dif: {}", current, volts[x], dif);
                temp_index = x;
            }
        }
        index = temp_index;
        current = volts[temp_index];
        path.push(current);
    }
    //println!("Path {:?}", path);
    path.reverse();
    Ok(path)
}
// Has to be sorted data
fn find_critical_path(mut volts: Vec<usize>) -> AoCResult<Vec<usize>> {
    //volts.push(0);
    volts.sort();
    let mut index = 0;
    let mut temp_index = 0;

    let mut current = volts[0];

    let mut path = Vec::new();
    path.push(current);

    while current != *AoCError::from_option(volts.last())? {
        for x in index..volts.len() {
            let dif = volts[x] - volts[index];
            if 0 < dif && dif <= 3 {
                //println!("Current: {}, X_Volt {}, Dif: {}", current, volts[x], dif);
                temp_index = x;
            }
        }
        index = temp_index;
        current = volts[temp_index];
        path.push(current);
    }
    //println!("Path {:?}", path);
    //path.reverse();
    Ok(path)
}

fn part_2d(volts: Vec<usize>) -> AoCResult<usize> {
    let mut possible: HashMap<usize, HashSet<Vec<usize>>> = HashMap::new();
    for volt in &volts {
        possible.insert(*volt, HashSet::new());
        AoCError::from_option(possible.get_mut(volt))?.insert(vec![*volt]);
    }
    for _target in 0..*AoCError::from_option(volts.last())? {
        for (value, _amount) in possible.iter() {
            let _count = *value;
            //while count < target {}
            //possible.insert(1, vec![1 as usize]);
        }
    }
    Ok(0)
}

fn part_2c(mut volts: Vec<usize>) -> AoCResult<usize> {
    volts.sort();
    let critical = find_critical_path(volts.clone())?;
    println!("Critical {:?}", critical);
    println!("Volts {:?}", volts);
    let mut iterations = 1;
    for crit_index in 0..critical.len() - 1 {
        let mut possibilities = 1;
        for volt in &volts {
            if volt >= &critical[crit_index + 1] {
                break;
            }
            if volt <= &critical[crit_index] {
            } else {
                possibilities += 1;
            }
        }
        println!(
            "   From {}, To {} Has {}",
            critical[crit_index],
            critical[crit_index + 1],
            possibilities
        );
        iterations *= AoCError::from_option(2_u32.checked_pow(possibilities - 1))? as usize;
    }
    Ok(iterations)
}

fn part_2_a(volts: &[usize]) -> usize {
    let mut iterations = 0;
    for index in 0..volts.len() {
        for sub in index..volts.len() {
            let dif = volts[sub] - volts[index];
            if 0 < dif && dif < 4 {
                iterations += 1;
            }
        }
    }
    iterations
}

fn part_2(volts: &[usize], current_index: usize, depth: usize, target: usize) -> usize {
    if current_index >= volts.len() {
        return 0;
    }
    let mut iterations = 1;
    let mut found = false;
    for index in (current_index + 1)..(current_index + 4) {
        if index >= volts.len() {
            if !found && volts[current_index] != 22 {
                return 0;
            }
            return iterations;
        }
        for _ in 0..depth {
            print!("    ");
        }
        let dif = volts[index] - volts[current_index];
        println!(
            "Testing index: {} {} {}",
            index, current_index, volts[index]
        );
        if 0 < dif && dif <= 3 {
            found = true;
            iterations += part_2(volts, index, depth + 1, target);
        }
    }
    if !found {
        println!("FAILED {}", depth);
        return 0;
    }
    for _ in 0..depth {
        print!("    ");
    }
    println!(
        "Current index: {}, Depth: {}, Result: {},",
        current_index, depth, iterations
    );
    iterations
}
#[cfg(test)]
mod tests {
    use crate::days::day10::{part_1, part_2c};
    use crate::misc::read_vec_ints;

    #[test]
    fn part_1a_test() {
        let data = read_vec_ints(String::from("Inputs/test10a.txt")).unwrap();
        let result = part_1(data);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 35);
    }
    #[test]
    fn part_1b_test() {
        let data = read_vec_ints(String::from("Inputs/test10b.txt")).unwrap();
        let result = part_1(data);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 220);
    }
    #[test]
    fn part_1_input() {
        let data = read_vec_ints(String::from("Inputs/input10.txt")).unwrap();
        let result = part_1(data);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 2760);
    }
    #[test]
    fn part_2a_test() {
        let mut data = read_vec_ints(String::from("Inputs/test10a.txt")).unwrap();
        data.push(0);
        data.sort();
        data.push(data.last().unwrap() + 3);

        let result = part_2c(data);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 8);
    }
    #[test]
    fn part_2b_test() {
        let mut data = read_vec_ints(String::from("Inputs/test10b.txt")).unwrap();
        data.push(0);
        data.sort();
        data.push(data.last().unwrap() + 3);
        let result = part_2c(data);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 19208);
    }
}
