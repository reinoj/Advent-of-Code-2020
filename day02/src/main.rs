use std::fs;

fn main() {
    let (lows, highs, letters, passwords) = get_input();
    let valid_count: i32 = count_valid_passwords(&lows, &highs, &letters, &passwords);
    let new_valid_count: i32 = count_valid_passwords2(&lows, &highs, &letters, &passwords);
    println!(
        "valid passwords: {}\nnew valid passwords: {}",
        valid_count, new_valid_count
    );
}

fn get_input() -> (Vec<i32>, Vec<i32>, Vec<char>, Vec<String>) {
    let content = fs::read_to_string("src/input.txt").expect("Error while reading file.");
    let str_list: Vec<&str> = content.split('\n').collect();
    let mut lows: Vec<i32> = Vec::new();
    let mut highs: Vec<i32> = Vec::new();
    let mut letters: Vec<char> = Vec::new();
    let mut passwords: Vec<String> = Vec::new();
    for line in str_list {
        if line == "" {
            continue;
        }
        let parts: Vec<&str> = line.split(' ').collect();
        let nums: Vec<&str> = parts[0].split('-').collect();
        lows.push(nums[0].trim().parse::<i32>().unwrap());
        highs.push(nums[1].trim().parse::<i32>().unwrap());

        let c: Vec<char> = parts[1].chars().collect();
        letters.push(c[0]);
        passwords.push(parts[2].trim().to_string());
    }
    return (lows, highs, letters, passwords);
}

fn count_valid_passwords(
    lows: &Vec<i32>,
    highs: &Vec<i32>,
    letters: &Vec<char>,
    passwords: &Vec<String>,
) -> i32 {
    let mut valid_count: i32 = 0;
    let mut i: usize = 0;
    let length: usize = passwords.len();
    while i < length {
        let mut pw_count: i32 = 0;
        for letter in passwords[i].chars() {
            if letter == letters[i] {
                pw_count += 1;
            }
        }
        if pw_count >= lows[i] && pw_count <= highs[i] {
            valid_count += 1;
        }
        i += 1;
    }
    return valid_count;
}

fn count_valid_passwords2(
    first: &Vec<i32>,
    second: &Vec<i32>,
    letters: &Vec<char>,
    passwords: &Vec<String>,
) -> i32 {
    let mut valid_count: i32 = 0;
    let mut i: usize = 0;
    let length: usize = passwords.len();
    while i < length {
        let pw: Vec<char> = passwords[i].chars().collect();
        let f: usize = first[i] as usize - 1;
        let s: usize = second[i] as usize - 1;
        let res: bool = match (f < pw.len(), s < pw.len()) {
            (true, true) => (pw[f] == letters[i]) ^ (pw[s] == letters[i]),
            (true, false) => pw[f] == letters[i],
            (false, true) => pw[s] == letters[i],
            _ => {
                i += 1;
                continue;
            }
        };
        if res {
            valid_count += 1;
        }

        // println!(
        //     "pw: {}\nf: {}\ts: {}\tletter: {}\nresult: {}",
        //     passwords[i], f, s, letters[i], res
        // );
        i += 1;
    }
    return valid_count;
}
