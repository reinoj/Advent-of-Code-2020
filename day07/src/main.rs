use std::fs;

fn main() {
    let rules: Vec<Vec<String>> = get_input();
    let num_bags: usize = how_many_bags(&rules);
    println!("Bags with at least one shiny gold bag: {}", num_bags);
    let bags_inside: usize = bags_in_sgb(&rules);
    println!("Bags inside shiny gold bag: {}", bags_inside);
}

fn get_input() -> Vec<Vec<String>> {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<&str> = content.split('\n').collect();
    let mut rules: Vec<Vec<String>> = Vec::new();

    let mut r: Vec<String> = Vec::new();
    for line in lines {
        let spl: Vec<&str> = line.split("contain").map(|s| s.trim()).collect();
        let content: Vec<&str> = spl[1].split(",").map(|s| s.trim()).collect();
        if content[0].as_bytes()[0].is_ascii_digit() {
            let mut end: usize = spl[0].find("bag").unwrap();
            r.push(spl[0][..end].trim().to_string());
            for c in content {
                end = c.find("bag").unwrap();
                r.push(c[..end].trim().to_string());
            }
            rules.push(r.clone());
            r = Vec::new();
        }
    }
    rules
}

fn how_many_bags(rules: &Vec<Vec<String>>) -> usize {
    let mut prev_len: usize = usize::MAX;
    let mut contains_sgb: Vec<String> = Vec::new();
    while prev_len != contains_sgb.len() {
        prev_len = contains_sgb.len();
        for rule in rules {
            for i in 1..rule.len() {
                if &rule[i][2..] == "shiny gold" || contains_sgb.contains(&rule[i][2..].to_string())
                {
                    if !contains_sgb.contains(&rule[0]) {
                        contains_sgb.push(rule[0].clone())
                    }
                }
            }
        }
    }
    contains_sgb.len()
}

fn bags_in_sgb(rules: &Vec<Vec<String>>) -> usize {
    let mut relevant_bags: Vec<(String, Vec<(String, usize)>)> = Vec::new();
    let mut look_for: Vec<String> = vec!["shiny gold".to_string()];
    while let Some(bag) = look_for.pop() {
        for rule in rules {
            if rule[0] == bag {
                let mut add_bag: bool = false;
                for rb in &relevant_bags {
                    if rb.0 == rule[0] {
                        add_bag = true;
                    }
                }
                if !add_bag {
                    let mut tmp: Vec<(String, usize)> = Vec::new();
                    for i in 1..rule.len() {
                        look_for.push(rule[i][2..].to_string());
                        tmp.push((
                            rule[i][2..].to_string(),
                            rule[i][..1].parse::<usize>().unwrap(),
                        ));
                    }
                    relevant_bags.push((rule[0].clone(), tmp));
                    break;
                }
            }
        }
    }

    count_bags(&relevant_bags, "shiny gold".to_string())
}

fn count_bags(relevant_bags: &Vec<(String, Vec<(String, usize)>)>, cur_bag: String) -> usize {
    let mut tot: usize = 0;
    for rb in relevant_bags {
        if rb.0 == cur_bag {
            for i in 0..rb.1.len() {
                tot += (rb.1[i].1 * count_bags(relevant_bags, rb.1[i].0.clone())) + rb.1[i].1;
                // let tmp: usize = count_bags(relevant_bags, rb.1[i].0.clone());
                // tot += (rb.1[i].1 * tmp) + rb.1[i].1;
                // println!("{}: {} * {}", rb.1[i].0, rb.1[i].1, tmp);
            }
        }
    }
    tot
}
