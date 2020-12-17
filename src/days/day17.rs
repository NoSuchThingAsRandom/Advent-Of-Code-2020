extern crate fxhash;
use crate::misc::read_vec_string;
use fxhash::FxHashMap;
use fxhash::FxHashSet;
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::iter::FromIterator;
use std::time::Instant;

pub fn run() {
    //let data = read_vec_string(String::from("Inputs/test17.txt")).unwrap();
    let data = read_vec_string(String::from("Inputs/input17.txt")).unwrap();
    //part_1(&data);
    let start = Instant::now();
    let res = part_2(&data);
    let time = start.elapsed().as_millis();
    println!("Hash: {}, Time: {}", res, time);
    let start = Instant::now();
    let res = part_2_bt(&data);
    let time = start.elapsed().as_millis();
    println!("BT: {}, Time: {}", res, time);
}

fn part_1(data: &[String]) -> usize {
    // Build initial state
    let mut cubes: HashSet<(isize, isize, isize)> = HashSet::new();
    for (y, line) in data.iter().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            if letter.eq(&'#') {
                cubes.insert((x as isize, y as isize, 0));
            }
        }
    }
    for pass in 0..6 {
        let mut visited: HashSet<(isize, isize, isize)> = HashSet::new();
        let mut new_cubes = HashSet::new();
        for test_cube in cubes.iter() {
            let mut neighbours = get_neighbours_3dim(*test_cube);
            neighbours.retain(|x| !visited.contains(x));
            neighbours.retain(|x| !cubes.contains(x));
            for n in &neighbours {
                if check_cube_3dim(*n, &cubes) {
                    new_cubes.insert(*n);
                }
                visited.insert(*n);
            }
            if check_cube_3dim(*test_cube, &cubes) {
                new_cubes.insert(*test_cube);
            }
            visited.insert(*test_cube);
        }
        cubes = new_cubes;
    }
    cubes.len()
}

fn part_2(data: &[String]) -> usize {
    // Build initial state
    let mut cubes: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    for (y, line) in data.iter().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            if letter.eq(&'#') {
                cubes.insert((x as isize, y as isize, 0, 0));
            }
        }
    }
    for pass in 0..6 {
        let mut visited: HashSet<(isize, isize, isize, isize)> = HashSet::new();
        let mut new_cubes = HashSet::new();
        for test_cube in cubes.iter() {
            let mut neighbours = get_neighbours_4dim(*test_cube);
            neighbours.retain(|x| !visited.contains(x));
            neighbours.retain(|x| !cubes.contains(x));
            for n in &neighbours {
                if check_cube_4dim(*n, &cubes) {
                    new_cubes.insert(*n);
                }
                visited.insert(*n);
            }
            if check_cube_4dim(*test_cube, &cubes) {
                new_cubes.insert(*test_cube);
            }
            visited.insert(*test_cube);
        }
        cubes = new_cubes;
    }
    cubes.len()
}
fn part_2_bt(data: &[String]) -> usize {
    // Build initial state
    let mut cubes: BTreeSet<(isize, isize, isize, isize)> = BTreeSet::new();
    for (y, line) in data.iter().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            if letter.eq(&'#') {
                cubes.insert((x as isize, y as isize, 0, 0));
            }
        }
    }
    for pass in 0..6 {
        let mut visited: BTreeSet<(isize, isize, isize, isize)> = BTreeSet::new();
        let mut new_cubes = BTreeSet::new();
        for test_cube in cubes.iter() {
            let mut neighbours = get_neighbours_4dim(*test_cube);
            neighbours.retain(|x| !visited.contains(x));
            neighbours.retain(|x| !cubes.contains(x));
            for n in &neighbours {
                if check_cube_4dim_bt(*n, &cubes) {
                    new_cubes.insert(*n);
                }
                visited.insert(*n);
            }
            if check_cube_4dim_bt(*test_cube, &cubes) {
                new_cubes.insert(*test_cube);
            }
            visited.insert(*test_cube);
        }
        cubes = new_cubes;
    }
    cubes.len()
}

fn check_cube_3dim(
    test_cube: (isize, isize, isize),
    cubes: &HashSet<(isize, isize, isize)>,
) -> bool {
    let neighbour_count = count_neighbours_3dim(test_cube, &cubes);
    if cubes.contains(&test_cube) {
        if neighbour_count == 2 || neighbour_count == 3 {
            return true;
        }
    } else if neighbour_count == 3 {
        return true;
    }
    false
}
fn check_cube_4dim(
    test_cube: (isize, isize, isize, isize),
    cubes: &HashSet<(isize, isize, isize, isize)>,
) -> bool {
    let neighbour_count = count_neighbours_4dim(test_cube, &cubes);
    if cubes.contains(&test_cube) {
        if neighbour_count == 2 || neighbour_count == 3 {
            return true;
        }
    } else if neighbour_count == 3 {
        return true;
    }
    false
}
fn check_cube_4dim_bt(
    test_cube: (isize, isize, isize, isize),
    cubes: &BTreeSet<(isize, isize, isize, isize)>,
) -> bool {
    let neighbour_count = count_neighbours_4dim_bt(test_cube, &cubes);
    if cubes.contains(&test_cube) {
        if neighbour_count == 2 || neighbour_count == 3 {
            return true;
        }
    } else if neighbour_count == 3 {
        return true;
    }
    false
}

fn count_neighbours_3dim(
    coords: (isize, isize, isize),
    cubes: &HashSet<(isize, isize, isize)>,
) -> usize {
    let mut count = 0;
    for x in &[-1, 0, 1] {
        for y in &[-1, 0, 1] {
            for z in &[-1, 0, 1] {
                if *x == 0 && *y == 0 && *z == 0 {
                    continue;
                }
                if cubes
                    .get(&(coords.0 + x, coords.1 + y, coords.2 + z))
                    .is_some()
                {
                    count += 1;
                }
            }
        }
    }
    count
}
fn count_neighbours_4dim(
    coords: (isize, isize, isize, isize),
    cubes: &HashSet<(isize, isize, isize, isize)>,
) -> usize {
    let mut count = 0;
    for x in &[-1, 0, 1] {
        for y in &[-1, 0, 1] {
            for z in &[-1, 0, 1] {
                for w in &[-1, 0, 1] {
                    if *x == 0 && *y == 0 && *z == 0 && *w == 0 {
                        continue;
                    }
                    if cubes
                        .get(&(coords.0 + x, coords.1 + y, coords.2 + z, coords.3 + w))
                        .is_some()
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
fn count_neighbours_4dim_bt(
    coords: (isize, isize, isize, isize),
    cubes: &BTreeSet<(isize, isize, isize, isize)>,
) -> usize {
    let mut count = 0;
    for x in &[-1, 0, 1] {
        for y in &[-1, 0, 1] {
            for z in &[-1, 0, 1] {
                for w in &[-1, 0, 1] {
                    if *x == 0 && *y == 0 && *z == 0 && *w == 0 {
                        continue;
                    }
                    if cubes
                        .get(&(coords.0 + x, coords.1 + y, coords.2 + z, coords.3 + w))
                        .is_some()
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn get_neighbours_3dim(coords: (isize, isize, isize)) -> Vec<(isize, isize, isize)> {
    let mut neighbours = Vec::new();
    for x in &[-1, 0, 1] {
        for y in &[-1, 0, 1] {
            for z in &[-1, 0, 1] {
                if *x == 0 && *y == 0 && *z == 0 {
                    continue;
                }
                neighbours.push((x + coords.0, y + coords.1, z + coords.2))
            }
        }
    }
    neighbours
}
fn get_neighbours_4dim(coords: (isize, isize, isize, isize)) -> Vec<(isize, isize, isize, isize)> {
    let mut neighbours = Vec::new();
    for x in &[-1, 0, 1] {
        for y in &[-1, 0, 1] {
            for z in &[-1, 0, 1] {
                for w in &[-1, 0, 1] {
                    if *x == 0 && *y == 0 && *z == 0 && *w == 0 {
                        continue;
                    }
                    neighbours.push((x + coords.0, y + coords.1, z + coords.2, w + coords.3))
                }
            }
        }
    }
    neighbours
}

#[cfg(test)]
mod tests {
    use crate::days::day17::{part_1, part_2};
    use crate::misc::read_vec_string;

    #[test]
    fn part_1_test() {
        let data = read_vec_string(String::from("Inputs/test17.txt")).unwrap();
        let result = part_1(&data);
        assert_eq!(result, 112);
    }

    #[test]
    fn part_2_test() {
        let data = read_vec_string(String::from("Inputs/test17.txt")).unwrap();
        let result = part_2(&data);
        assert_eq!(result, 848);
    }

    #[test]
    fn part_1_input() {
        let data = read_vec_string(String::from("Inputs/input17.txt")).unwrap();
        let result = part_1(&data);
        assert_eq!(result, 273);
    }

    #[test]
    fn part_2_input() {
        let data = read_vec_string(String::from("Inputs/input17.txt")).unwrap();
        let result = part_2(&data);
        assert_eq!(result, 1504);
    }
}

fn display_cube_state_2dim(cubes: &HashSet<(isize, isize, isize)>) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut x_zero = 0;
    let mut y_zero = 0;
    let mut output: VecDeque<VecDeque<char>> = VecDeque::new();
    output.push_front(VecDeque::new());
    for coord in cubes {
        if coord.2 != 0 {
            continue;
        }
        if coord.0 < 0 {
            for sub in output.iter_mut() {
                for _ in 0..((coord.0 + x_zero).abs()) {
                    sub.push_front('.');
                }
            }
            x_zero += (coord.0 + x_zero).abs();
        } else {
            let mut to_add = coord.0 - max_x;
            to_add += 1;
            if to_add < 0 {
                to_add = 0;
            }
            max_x += to_add;
            let to_add = to_add as usize;
            for sub in output.iter_mut() {
                for _ in 0..to_add {
                    sub.push_back('.');
                }
            }
        }
        if coord.1 < 0 {
            for _ in 0..((coord.1 + y_zero).abs()) {
                let mut new_string = VecDeque::new();
                for _ in 0..output.get(0).unwrap().len() {
                    new_string.push_back('.');
                }
                output.push_front(new_string);
            }
            y_zero += (coord.1 + y_zero).abs();
        } else {
            let mut to_add = coord.1 - max_y;
            //to_add += 1;
            if to_add < 0 {
                to_add = 0;
            }
            max_y += to_add;
            let to_add = to_add as usize;
            for _ in 0..to_add {
                let mut new_string = VecDeque::new();
                for _ in 0..output.get(0).unwrap().len() {
                    new_string.push_back('.');
                }
                output.push_back(new_string);
            }
        }
        /*        println!("Adding: {:?}", coord);
        println!("X: {} {}", output.get(0).unwrap().len(), x_zero);
        println!("Y: {} {}", output.len(), y_zero);
        println!("{:?}", output);*/
        let mut c = output
            .get_mut((coord.1 + y_zero) as usize)
            .unwrap()
            .get_mut((coord.0 + x_zero) as usize)
            .unwrap();
        *c = '#';
        //println!("{:?}\n\n-------------\n", output);
    }

    for y in output {
        for x in y {
            print!("{}", x);
        }
        print!("\n");
    }
}

fn display_cube_state_3dim(cubes: &HashSet<(isize, isize, isize)>) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    let mut x_zero = 0;
    let mut y_zero = 0;
    let mut z_zero = 0;
    let mut output: VecDeque<VecDeque<VecDeque<char>>> = VecDeque::new();
    let mut a = VecDeque::new();
    a.push_back(VecDeque::new());
    output.push_front(a);
    for coord in cubes {
        if coord.2 < 0 {
            for _ in 0..((coord.2 + z_zero).abs()) {
                let mut new = VecDeque::new();
                for _sub_y in 0..output.get(0).unwrap().len() {
                    let mut new_y = VecDeque::new();
                    for _sub_z in 0..output.get(0).unwrap().get(0).unwrap().len() {
                        new_y.push_back('.');
                    }
                    new.push_back(new_y);
                }
                output.push_back(new);
            }
            z_zero += (coord.2 + z_zero).abs();
        } else {
            let mut to_add = coord.2 - max_z;
            to_add += 1;
            if to_add < 0 {
                to_add = 0;
            }
            max_z += to_add;
            let to_add = to_add as usize;
            for _ in 0..to_add {
                let mut new = VecDeque::new();
                for _sub_y in 0..output.get(0).unwrap().len() {
                    let mut new_y = VecDeque::new();
                    for _sub_z in 0..output.get(0).unwrap().get(0).unwrap().len() {
                        new_y.push_back('.');
                    }
                    new.push_back(new_y);
                }
                output.push_front(new);
            }
        }
        if coord.0 < 0 {
            for sub_z in output.iter_mut() {
                for sub_y in sub_z.iter_mut() {
                    for _ in 0..((coord.0 + x_zero).abs()) {
                        sub_y.push_front('.');
                    }
                }
            }
            x_zero += (coord.0 + x_zero).abs();
        } else {
            let mut to_add = coord.0 - max_x;
            to_add += 1;
            if to_add < 0 {
                to_add = 0;
            }
            max_x += to_add;
            let to_add = to_add as usize;
            for sub_z in output.iter_mut() {
                for sub_y in sub_z.iter_mut() {
                    for _ in 0..to_add {
                        sub_y.push_back('.');
                    }
                }
            }
        }
        if coord.1 < 0 {
            for sub_z in output.iter_mut() {
                for _ in 0..((coord.1 + y_zero).abs()) {
                    let mut new_string = VecDeque::new();
                    for _ in 0..sub_z.get(0).unwrap().len() {
                        new_string.push_back('.');
                    }
                    sub_z.push_front(new_string);
                }
            }
            y_zero += (coord.1 + y_zero).abs();
        } else {
            let mut to_add = coord.1 - max_y;
            //to_add += 1;
            if to_add < 0 {
                to_add = 0;
            }
            max_y += to_add;
            let to_add = to_add as usize;
            for sub_z in output.iter_mut() {
                for _ in 0..to_add {
                    let mut new_string = VecDeque::new();
                    for _ in 0..sub_z.get(0).unwrap().len() {
                        new_string.push_back('.');
                    }
                    sub_z.push_back(new_string);
                }
            }
        }
        /*        println!("Adding: {:?}", coord);
        println!(
            "X: {} {}",
            output.get(0).unwrap().get(0).unwrap().len(),
            x_zero
        );
        println!("Y: {} {}", output.get(0).unwrap().len(), y_zero);
        println!("Z: {} {}", output.len(), z_zero);
        println!("{:?}", output);*/
        let mut c = output
            .get_mut((coord.2 + z_zero) as usize)
            .unwrap()
            .get_mut((coord.1 + y_zero) as usize)
            .unwrap()
            .get_mut((coord.0 + x_zero) as usize)
            .unwrap();
        *c = '#';
        //println!("{:?}\n\n-------------\n", output);
    }
    for (index, z) in output.iter().enumerate() {
        println!("Z: {}", index);
        for y in z {
            let mut out = String::new();
            out.push_str("    ");
            for x in y {
                out.push(*x);
            }
            println!("{}", out);
        }
    }
}
