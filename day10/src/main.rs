use std::collections::HashMap;
use std::fs;

fn main() {
    let mut adapters: Vec<i64> = get_input();
    let (one_difs, three_difs): (i64, i64) = jolt_differences(&adapters);
    println!(
        "One-difs: {}\tThree-difs: {}\tProduct: {}",
        one_difs,
        three_difs,
        one_difs * three_difs
    );
    adapters.insert(0, 0);
    let mut hash: HashMap<usize, i64> = HashMap::new();
    let total: i64 = count_all_possibilities(&adapters, &mut hash, adapters.len() - 1);
    println!("accumulator: {}", total);
}

fn get_input() -> Vec<i64> {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<&str> = content.split('\n').map(|s| s.trim()).collect();
    let mut adapters: Vec<i64> = Vec::new();
    for line in lines {
        adapters.push(line.parse::<i64>().unwrap());
    }
    adapters.sort();
    adapters
}

fn jolt_differences(adapters: &Vec<i64>) -> (i64, i64) {
    let mut one_difs: i64 = 0;
    let mut three_difs: i64 = 1;
    let mut cur_joltage: i64 = 0;
    for &a in adapters {
        match a - cur_joltage {
            1 => one_difs += 1,
            3 => three_difs += 1,
            _ => println!("not a difference of 1 or 3"),
        }
        cur_joltage = a;
    }
    (one_difs, three_difs)
}

fn count_all_possibilities(
    adapters: &Vec<i64>,
    hash: &mut HashMap<usize, i64>,
    cur_index: usize,
) -> i64 {
    if cur_index == 0 {
        return 1;
    }
    if let Some(v) = hash.get(&cur_index) {
        return *v;
    }
    let mut acc: i64 = 0;
    for i in (0..cur_index).rev() {
        if adapters[cur_index] - adapters[i] <= 3 {
            if let Some(v) = hash.get(&i) {
                acc += v;
            } else {
                acc += count_all_possibilities(adapters, hash, i);
            }
        } else {
            break;
        }
    }
    hash.insert(cur_index, acc);
    acc
}
