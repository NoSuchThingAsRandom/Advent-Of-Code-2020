use crate::misc::error::{AoCError, AoCResult};
use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct TicketData {
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
    parameters: HashMap<String, BTreeSet<usize>>,
}
impl TicketData {
    fn parse_ticket_data(filename: String) -> AoCResult<TicketData> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut fields = HashMap::new();
        let mut my_ticket: Vec<usize> = Vec::new();
        let mut nearby_tickets: Vec<Vec<usize>> = Vec::new();
        let mut is_fields = true;
        let mut is_my_ticket = false;
        let mut is_nearby_tickets = false;
        for line in reader.lines() {
            let line = line?;
            if is_fields {
                if line.eq("") {
                    is_fields = false;
                    is_my_ticket = true;
                    continue;
                }
                let mut split = line.split(':');
                let name = AoCError::from_option(split.next())?;
                let parameters = split.next().unwrap().split(" or ");
                let mut values = BTreeSet::new();
                for param in parameters {
                    let mut split_param = param.trim().split('-');
                    let start = split_param.next().unwrap().parse::<usize>().unwrap();
                    let end = split_param.next().unwrap().parse::<usize>().unwrap();
                    for x in start..(end + 1) {
                        values.insert(x);
                    }
                }
                fields.insert(name.to_string(), values);
            } else if is_my_ticket && !line.contains(':') {
                is_my_ticket = false;
                is_nearby_tickets = true;
                my_ticket = line
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
            } else if is_nearby_tickets && !line.contains(':') && !line.is_empty() {
                is_my_ticket = false;
                nearby_tickets.push(
                    line.split(',')
                        .map(|x| x.trim().parse::<usize>().unwrap())
                        .collect(),
                );
            }
        }
        Ok(TicketData {
            my_ticket,
            nearby_tickets,
            parameters: fields,
        })
    }
    fn nearby_tickets_to_string(&self) -> String {
        let mut output = String::new();
        for ticket in &self.nearby_tickets {
            output.push_str(format!("      {:?}\n", ticket).as_str());
        }
        output
    }
    fn parameters_to_string(&self) -> String {
        let mut output = String::new();
        for (field, values) in &self.parameters {
            output.push_str(format!("      {} : {:?}\n", field, values).as_str());
        }
        output
    }
    fn count_error_rate(&self) -> usize {
        let mut valid_values = BTreeSet::new();
        for value in self.parameters.values() {
            valid_values.append(&mut (value.clone()));
        }
        let mut error_count = 0;
        for ticket in &self.nearby_tickets {
            for x in ticket {
                if !valid_values.contains(x) {
                    error_count += x;
                }
            }
        }
        error_count
    }
    fn discard_invalid_tickets(&mut self) {
        let mut valid_values = BTreeSet::new();
        for value in self.parameters.values() {
            valid_values.append(&mut (value.clone()));
        }
        self.nearby_tickets.retain(|ticket| {
            let mut is_valid = true;
            for x in ticket {
                if !valid_values.contains(x) {
                    is_valid = false;
                }
            }
            is_valid
        });
    }

    fn find_field_positions(&self) -> HashMap<usize, &String> {
        let mut possible_field_positions = HashMap::new();
        for (field, values) in &self.parameters {
            let mut possible_positions = Vec::new();
            for index in 0..(&self.nearby_tickets.get(0)).unwrap().len() {
                let mut valid = true;
                for ticket in &self.nearby_tickets {
                    if let Some(x) = ticket.get(index) {
                        if !values.contains(x) {
                            valid = false;
                        }
                    } else {
                        valid = false;
                    }
                }
                if valid {
                    possible_positions.push(index);
                }
            }
            possible_field_positions.insert(field, possible_positions);
        }
        let mut taken = HashMap::new();
        let mut index = 0;
        while !possible_field_positions.is_empty() {
            /*            println!(
                "Pass: {}, Remaining Indexes: {}",
                index,
                possible_field_positions.len()
            );*/
            possible_field_positions.retain(|field, possible| {
                if possible.len() == 1 {
                    let index = *possible.get(0).unwrap();
                    if taken.contains_key(&index) {
                        panic!("Two conflicting field positions!");
                    }
                    taken.insert(index, *field);
                    false
                } else {
                    possible.retain(|x| !taken.contains_key(x));
                    true
                }
            });
            index += 1;
            if index >= 50 {
                println!("Failed to assign values!");
                break;
            }
        }
        taken
    }
}
impl Display for TicketData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "My Ticket: {:?}\n\nNearby Tickets:\n{}\nParameters:\n{}",
            self.my_ticket,
            self.nearby_tickets_to_string(),
            self.parameters_to_string()
        )
    }
}
fn part_1(file: String) -> usize {
    let ticket_data = TicketData::parse_ticket_data(file).unwrap();
    ticket_data.count_error_rate()
}
fn part_2(file: String) -> usize {
    let mut ticket_data = TicketData::parse_ticket_data(file).unwrap();
    ticket_data.discard_invalid_tickets();
    let positions = ticket_data.find_field_positions();
    let mut output = 1;
    for (index, field) in &positions {
        if field.contains("departure") {
            output *= ticket_data.my_ticket.get(*index).unwrap();
        }
    }
    output
}
pub(crate) fn run() {
    let file = String::from("Inputs/input16.txt");
    println!("Part 1: {}", part_1(file.clone()));
    println!("Part 2: {}", part_2(file));
}
#[cfg(test)]
mod tests {
    use crate::days::day16::{part_1, part_2};

    #[test]
    fn part_1_test() {
        let file = String::from("Inputs/test16a.txt");
        assert_eq!(part_1(file), 71);
    }
    #[test]
    fn part_1_input() {
        let file = String::from("Inputs/input16.txt");
        assert_eq!(part_1(file), 29878);
    }
    #[test]
    fn part_2_test() {
        let file = String::from("Inputs/test16b.txt");
        assert_eq!(part_2(file), 1);
    }
    #[test]
    fn part_2_input() {
        let file = String::from("Inputs/input16.txt");
        assert_eq!(part_2(file), 855438643439);
    }
}
