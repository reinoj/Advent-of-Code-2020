use std::fs;

fn main() {
    let nums: Vec<u64> = get_input();
    let the_num: u64 = find_num(&nums);
    println!("First number: {}", the_num);
    let encryption_weakness: u64 = find_encryption_weakness(&nums, the_num);
    println!("Encryption weakness: {}", encryption_weakness);
}

fn get_input() -> Vec<u64> {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<&str> = content.split('\n').map(|s| s.trim()).collect();
    let mut nums: Vec<u64> = Vec::new();
    for line in lines {
        nums.push(line.parse::<u64>().unwrap());
    }
    nums
}

fn find_num(nums: &Vec<u64>) -> u64 {
    let mut low: usize = 0;
    let mut high: usize = 25;
    loop {
        if high == nums.len() {
            break u64::MIN;
        }
        let mut is_the_num: bool = true;
        let num: u64 = nums[high];
        for i in low..high - 1 {
            for j in low + 1..high {
                if i == j {
                    continue;
                }
                if nums[i] + nums[j] == num {
                    if nums[i] != nums[j] {
                        is_the_num = false;
                        break;
                    }
                }
                if !is_the_num {
                    break;
                }
            }
        }
        if is_the_num {
            break num;
        }
        low += 1;
        high += 1;
    }
}

fn find_encryption_weakness(nums: &Vec<u64>, invalid_num: u64) -> u64 {
    let mut size: usize = 2;
    let mut i: usize = 0;
    loop {
        let mut cur_total = 0;
        for ind in i..i + size {
            cur_total += nums[ind];
        }
        if cur_total == invalid_num {
            let min: &u64 = nums[i..i + size].iter().min().unwrap();
            let max: &u64 = nums[i..i + size].iter().max().unwrap();
            break min + max;
        }
        if i + size + 1 == nums.len() {
            i = 0;
            size += 1;
        } else {
            i += 1;
        }
    }
}
