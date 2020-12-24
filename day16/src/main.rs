use std::collections::HashMap;
use std::fs;

fn main() {
    let (rules, ticket, nearby_tickets) = get_input();
    let error_rate: u64 = ticket_scan_error_rate(&rules, &nearby_tickets);
    println!("Error rate: {}", error_rate);
    let product: u64 = product_of_departures(&rules, &ticket, &nearby_tickets);
    println!("Product: {}", product);
}

fn get_input() -> (Vec<Vec<Vec<u64>>>, Vec<u64>, Vec<Vec<u64>>) {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<&str> = content.split('\n').map(|s| s.trim()).collect();
    let mut rules: Vec<Vec<Vec<u64>>> = Vec::new();
    let mut ticket: Vec<u64> = Vec::new();
    let mut nearby_tickets: Vec<Vec<u64>> = Vec::new();
    let mut mode: u8 = 0;
    for line in lines {
        match mode {
            0 => {
                if line == "your ticket:" {
                    mode += 1;
                } else {
                    if !line.is_empty() {
                        let field_range: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
                        let ranges: Vec<&str> =
                            field_range[1].split("or").map(|s| s.trim()).collect();
                        let mut lows_highs: Vec<Vec<u64>> = Vec::new();
                        for range in ranges {
                            let lh: Vec<&str> = range.split('-').map(|s| s.trim()).collect();
                            lows_highs.push(vec![
                                lh[0].parse::<u64>().unwrap(),
                                lh[1].parse::<u64>().unwrap(),
                            ]);
                        }
                        rules.push(lows_highs);
                    }
                }
            }
            1 => {
                if line == "nearby tickets:" {
                    mode += 1;
                } else {
                    if !line.is_empty() {
                        let nums: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                        for num in nums {
                            ticket.push(num.parse::<u64>().unwrap());
                        }
                    }
                }
            }
            2 => {
                let mut t: Vec<u64> = Vec::new();
                let nums: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                for num in nums {
                    t.push(num.parse::<u64>().unwrap());
                }
                nearby_tickets.push(t);
            }
            _ => println!("Shouldn't get this."),
        }
    }
    (rules, ticket, nearby_tickets)
}

fn ticket_scan_error_rate(rules: &Vec<Vec<Vec<u64>>>, nearby_tickets: &Vec<Vec<u64>>) -> u64 {
    let mut error_rate: u64 = 0;
    for i in 0..nearby_tickets.len() {
        for j in 0..nearby_tickets[i].len() {
            let mut is_valid: bool = false;
            for ri in 0..rules.len() {
                is_valid = is_valid
                    || (nearby_tickets[i][j] >= rules[ri][0][0]
                        && nearby_tickets[i][j] <= rules[ri][0][1])
                    || (nearby_tickets[i][j] >= rules[ri][1][0]
                        && nearby_tickets[i][j] <= rules[ri][1][1]);
            }
            if !is_valid {
                error_rate += nearby_tickets[i][j];
            }
        }
    }
    error_rate
}

fn product_of_departures(
    rules: &Vec<Vec<Vec<u64>>>,
    ticket: &Vec<u64>,
    nearby_tickets: &Vec<Vec<u64>>,
) -> u64 {
    let mut invalid_index: Vec<usize> = Vec::new();
    for i in 0..nearby_tickets.len() {
        for j in 0..nearby_tickets[i].len() {
            let mut is_valid: bool = false;
            for ri in 0..rules.len() {
                is_valid = is_valid
                    || (nearby_tickets[i][j] >= rules[ri][0][0]
                        && nearby_tickets[i][j] <= rules[ri][0][1])
                    || (nearby_tickets[i][j] >= rules[ri][1][0]
                        && nearby_tickets[i][j] <= rules[ri][1][1]);
            }
            if !is_valid {
                invalid_index.push(i);
            }
        }
    }
    let mut invalid_rule_positions: Vec<Vec<usize>> = vec![Vec::new(); rules.len()];
    for i in 0..nearby_tickets.len() {
        if !invalid_index.contains(&i) {
            for j in 0..nearby_tickets[i].len() {
                for ri in 0..rules.len() {
                    if !((nearby_tickets[i][j] >= rules[ri][0][0]
                        && nearby_tickets[i][j] <= rules[ri][0][1])
                        || (nearby_tickets[i][j] >= rules[ri][1][0]
                            && nearby_tickets[i][j] <= rules[ri][1][1]))
                    {
                        if !invalid_rule_positions[ri].contains(&j) {
                            invalid_rule_positions[ri].push(j);
                        }
                    }
                }
            }
        }
    }
    let mut rule_to_position: HashMap<usize, usize> = HashMap::new();
    let mut pos_set: Vec<usize> = (0..20).collect();
    while !pos_set.is_empty() {
        println!(
            "pos_set len: {}\nrule_to_position len: {}",
            pos_set.len(),
            rule_to_position.len()
        );
        for i in 0..invalid_rule_positions.len() {
            if invalid_rule_positions[i].len() == pos_set.len() - 1 {
                for j in 0..pos_set.len() {
                    // println!("{}: {}", j, !pos_set.contains(&pos_set[j]));
                    if !invalid_rule_positions[i].contains(&pos_set[j]) {
                        rule_to_position.insert(i, pos_set[j]);
                        pos_set.remove(j);
                        break;
                    }
                }
                break;
            }
        }
        if pos_set.len() == 20 {
            break;
        }
    }

    let mut product: u64 = 1;
    for i in 0..6 {
        match rule_to_position.get(&i) {
            Some(p) => product *= ticket[*p],
            None => println!("{} not in hashmap.", i),
        }
    }
    product
}
