use crate::misc::error::AoCResult;
use crate::misc::read_vec_string;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() -> AoCResult<usize> {
    println!("Test");
    let original = read_vec_string(String::from("Inputs/input07.txt")).unwrap();
    println!("Bags: {:?}", parse_bags(&original));
    println!("Possible bags: {}", part_1(&original).unwrap());
    println!("Bag count: {}", part_2(&original).unwrap());
    Ok(0)
}

fn parse_bags(data: &[String]) -> AoCResult<HashMap<String, Vec<(String, u16)>>> {
    let mut bags: HashMap<String, Vec<(String, u16)>> = HashMap::new();
    for line in data {
        let bag_split = line
            .to_lowercase()
            .replace("bags", "")
            .replace("bag", "")
            .replace(".", "")
            .replace(" ", "");
        let mut bag_split = bag_split.split("contain");
        let selected_bag = bag_split.next().unwrap();
        let mut contained_bags: Vec<(String, u16)> = bag_split
            .next()
            .unwrap()
            .split(',')
            .map(|x| {
                let mut chars = x.chars();
                let num_char = chars.next().unwrap();
                let num = if num_char.is_numeric() {
                    num_char.to_string().parse().unwrap()
                } else {
                    0
                };
                (chars.collect(), num)
            })
            .collect();
        if let Some(bag) = bags.get_mut(selected_bag) {
            bag.append(&mut contained_bags)
        } else {
            bags.insert(selected_bag.to_string(), contained_bags);
        }
    }
    Ok(bags)
}

fn find_containers(start_bag: String, bags: &HashMap<String, Vec<(String, u16)>>) -> Vec<String> {
    let mut containers = Vec::new();
    for (parent, children) in bags.iter() {
        for child in children {
            if child.0.eq(&start_bag) {
                containers.push(parent.to_string());
                break;
            }
        }
    }
    containers
}

pub fn part_1(data: &[String]) -> AoCResult<usize> {
    let bags = parse_bags(data)?;
    let mut output = HashSet::new();
    let mut to_process = VecDeque::from(find_containers(String::from("shinygold"), &bags));
    while !to_process.is_empty() {
        let bag = to_process.pop_front().unwrap();
        output.insert(bag.to_string());
        for sub_bag in find_containers(bag.to_string(), &bags) {
            if !output.contains(sub_bag.as_str()) {
                to_process.push_back(sub_bag.to_string());
            }
        }
    }
    Ok(output.len())
}
fn recurse(current_bag: &str, bags: &HashMap<String, Vec<(String, u16)>>, depth: u16) -> u16 {
    if current_bag.eq("oother") {
        for _ in 0..depth {
            print!("    ");
        }
        println!("1");
        return 1;
    }
    let mut total = 1;
    for bag in bags.get(current_bag).unwrap() {
        if bag.0.eq("oother") {
            total += 0;
        } else {
            let count = recurse(&bag.0, &bags, depth + 1) * bag.1;
            for _ in 0..depth {
                print!("    ");
            }
            println!(":  {}, {}, {}", bag.0, bag.1, count);
            total += count;
        }
    }
    for _ in 0..depth {
        print!("    ");
    }
    println!("total {}", total);
    total
}
pub fn part_2(data: &[String]) -> AoCResult<usize> {
    let bags = parse_bags(data)?;
    Ok(recurse(&"shinygold".to_string(), &bags, 0) as usize - 1)
}
