use std::collections::HashMap;
use std::fs;

fn main() {
    let mut masks_and_mems: Vec<(String, Vec<usize>, Vec<u64>)> = get_input();
    let sum: u64 = sum_of_memory(&mut masks_and_mems);
    println!("sum: {}", sum);
    masks_and_mems = get_input();
    let sum2: u64 = version_two(&mut masks_and_mems);
    println!("v2 sum: {}", sum2);
}

fn get_input() -> Vec<(String, Vec<usize>, Vec<u64>)> {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<&str> = content.split('\n').map(|s| s.trim()).collect();
    let mut masks_and_mems: Vec<(String, Vec<usize>, Vec<u64>)> = Vec::new();

    let mut is_first: bool = true;
    let mut mam: (String, Vec<usize>, Vec<u64>) = (String::new(), Vec::new(), Vec::new());
    for line in lines {
        if line.contains("mask") {
            if !is_first {
                masks_and_mems.push(mam.clone());
                mam = (String::new(), Vec::new(), Vec::new());
            } else {
                is_first = false;
            }
            mam.0 = line[7..].to_string();
        } else {
            let l: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
            mam.1
                .push(l[0].trim_end_matches(']')[4..].parse::<usize>().unwrap());
            mam.2.push(l[1].parse::<u64>().unwrap());
        }
    }
    masks_and_mems.push(mam.clone());

    masks_and_mems
}

fn sum_of_memory(masks_and_mems: &mut Vec<(String, Vec<usize>, Vec<u64>)>) -> u64 {
    let mut mem_hash: HashMap<usize, u64> = HashMap::new();
    for i in 0..masks_and_mems.len() {
        let mut memv_cpy: Vec<u64> = masks_and_mems[i].2.clone();
        let mut cur_bit: u64 = (2 as u64).pow(35);
        for c in masks_and_mems[i].0.clone().chars() {
            for j in 0..memv_cpy.len() {
                let quot: u64 = memv_cpy[j] / cur_bit;
                if c != 'X' {
                    if c == '0' && quot == 1 {
                        masks_and_mems[i].2[j] -= cur_bit;
                    } else if c == '1' && quot == 0 {
                        masks_and_mems[i].2[j] += cur_bit;
                    }
                }
                if quot != 0 {
                    memv_cpy[j] -= cur_bit;
                }
            }
            cur_bit /= 2;
        }
        for j in 0..masks_and_mems[i].1.len() {
            if mem_hash.contains_key(&masks_and_mems[i].1[j]) {
                *mem_hash.entry(masks_and_mems[i].1[j]).or_insert(0) = masks_and_mems[i].2[j];
            } else {
                mem_hash.insert(masks_and_mems[i].1[j], masks_and_mems[i].2[j]);
            }
        }
    }

    let mut sum: u64 = 0;
    for (_, v) in mem_hash.iter() {
        sum += v;
    }
    sum
}

fn version_two(masks_and_mems: &mut Vec<(String, Vec<usize>, Vec<u64>)>) -> u64 {
    let mut mem_hash: HashMap<usize, u64> = HashMap::new();
    for i in 0..masks_and_mems.len() {
        let mut masked_address: Vec<Vec<char>> = masks_and_mems[i]
            .1
            .iter()
            .map(|u| format!("{:#038b}", u).clone().chars().collect())
            .collect();
        for j in 0..masks_and_mems[i].0.len() {
            match masks_and_mems[i].0.chars().nth(j) {
                Some('0') => {}
                Some('1') => masked_address.iter_mut().for_each(|vc| vc[j + 2] = '1'),
                Some('X') => masked_address.iter_mut().for_each(|vc| vc[j + 2] = 'X'),
                _ => println!(
                    "Non - 0, 1, or X char: {}",
                    masks_and_mems[i].0.chars().nth(j).unwrap()
                ),
            }
        }
        for j in 0..masked_address.len() {
            let mut ma_num: Vec<usize> = vec![0];
            let mut cur_bit: usize = (2 as usize).pow(35);
            for k in 2..masked_address[j].len() {
                match masked_address[j][k] {
                    '1' => ma_num = ma_num.iter().map(|n| n + cur_bit).collect(),
                    'X' => {
                        let mut tmp: Vec<usize> = ma_num.iter().map(|n| n + cur_bit).collect();
                        tmp.append(&mut ma_num);
                        ma_num = tmp;
                    }
                    _ => {
                        if masked_address[j][k] != '0' {
                            println!("Non - 0, 1, or X char: {}", masked_address[j][k]);
                        }
                    }
                }
                cur_bit /= 2;
            }
            for k in 0..ma_num.len() {
                if mem_hash.contains_key(&k) {
                    *mem_hash.entry(ma_num[k]).or_insert(0) = masks_and_mems[i].2[j];
                } else {
                    mem_hash.insert(ma_num[k], masks_and_mems[i].2[j]);
                }
            }
        }
    }

    let mut sum: u64 = 0;
    for (_, v) in mem_hash.iter() {
        sum += v;
    }
    sum
}
