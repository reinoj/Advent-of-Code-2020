use std::fs;

fn main() {
    let lines: Vec<String> = get_input();
    let highest_sid: u32 = get_highest_sid(&lines);
    println!("Highest Seat ID: {}", highest_sid);
    let mut all_sids: Vec<u32> = get_all_sids(&lines);
    all_sids.sort();
    let mut prev: u32 = all_sids[0];
    for i in 1..all_sids.len() {
        if all_sids[i] - prev != 1 {
            println!("Your seat is: {}", all_sids[i] - 1);
            break;
        }
        prev = all_sids[i];
    }
}

fn get_input() -> Vec<String> {
    let content: String = fs::read_to_string("src/input").expect("Error reading file.");
    let str_lines: Vec<&str> = content.split('\n').collect();
    str_lines.iter().map(|s| s.trim().to_string()).collect()
    // let lines: Vec<String> = str_lines.iter().map(|s| s.trim().to_string()).collect();
    // lines
}

fn get_highest_sid(lines: &Vec<String>) -> u32 {
    let mut highest: u32 = 0;
    for line in lines {
        let mut row: u32 = 0;
        let mut row_change: u32 = 64;

        let mut col: u32 = 0;
        let mut col_change: u32 = 4;

        let l: Vec<char> = line.chars().collect();
        for i in 0..10 {
            match l[i] {
                'F' => {
                    row_change /= 2;
                }
                'B' => {
                    row += row_change;
                    row_change /= 2;
                }
                'L' => {
                    col_change /= 2;
                }
                'R' => {
                    col += col_change;
                    col_change /= 2;
                }
                _ => {
                    println!("Character isn't 'F', 'B', 'R', or 'L'.");
                    continue;
                }
            }
        }
        let sid: u32 = (row * 8) + col;
        if sid > highest {
            highest = sid;
        }
    }
    highest
}

fn get_all_sids(lines: &Vec<String>) -> Vec<u32> {
    let mut all_sids: Vec<u32> = Vec::new();
    for line in lines {
        let mut row: u32 = 0;
        let mut row_change: u32 = 64;

        let mut col: u32 = 0;
        let mut col_change: u32 = 4;

        let l: Vec<char> = line.chars().collect();
        for i in 0..10 {
            match l[i] {
                'F' => {
                    row_change /= 2;
                }
                'B' => {
                    row += row_change;
                    row_change /= 2;
                }
                'L' => {
                    col_change /= 2;
                }
                'R' => {
                    col += col_change;
                    col_change /= 2;
                }
                _ => {
                    println!("Character isn't 'F', 'B', 'R', or 'L'.");
                    continue;
                }
            }
        }
        let sid: u32 = (row * 8) + col;
        all_sids.push(sid);
    }
    all_sids
}
