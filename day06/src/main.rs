use std::fs;

fn main() {
    let groups: Vec<Vec<String>> = get_input();
    let count: u32 = get_sum_of_counts(&groups);
    let count2: u32 = get_sum_of_counts2(&groups);
    println!("Count: {}\n2nd Count: {}", count, count2);
}

fn get_input() -> Vec<Vec<String>> {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<&str> = content.split('\n').map(|s| s.trim()).collect();
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut g: Vec<String> = Vec::new();
    for line in lines {
        if line.is_empty() {
            groups.push(g.clone());
            g = Vec::new();
        } else {
            g.push(line.to_string());
        }
    }
    if !g.is_empty() {
        groups.push(g.clone());
    }

    groups
}

fn get_sum_of_counts(groups: &Vec<Vec<String>>) -> u32 {
    let mut count: u32 = 0;
    for group in groups {
        let mut answers: Vec<bool> = vec![false; 26];
        for line in group {
            for &c in line.as_bytes() {
                let i: usize = c as usize - 97;
                if !answers[i] {
                    answers[i] = true;
                }
            }
        }
        count += answers.iter().fold(0, |acc, b| match b {
            true => acc + 1,
            false => acc,
        });
    }

    count
}

fn get_sum_of_counts2(groups: &Vec<Vec<String>>) -> u32 {
    let mut count: u32 = 0;
    for group in groups {
        let mut answers: Vec<u8> = vec![0; 26];
        let num_people: u8 = group.len() as u8;
        for line in group {
            for &c in line.as_bytes() {
                let i: usize = c as usize - 97;
                answers[i] += 1;
            }
        }
        count += answers
            .iter()
            .fold(0, |acc, &n| if n == num_people { acc + 1 } else { acc });
    }

    count
}
