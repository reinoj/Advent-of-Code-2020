use std::fs;

fn main() {
    let nums_list: Vec<i64> = get_input();
    let res_two: i64 = multiply_sum(&nums_list);
    let res_three: i64 = multiply_sum_three(&nums_list);
    println!("Results: {}, {}", res_two, res_three);
}

fn get_input() -> Vec<i64> {
    let content = fs::read_to_string("src/input.txt").expect("Error while reading file.");
    let str_list: Vec<&str> = content.split('\n').collect();
    let mut nums_list: Vec<i64> = Vec::new();
    for str_num in &str_list {
        nums_list.push(str_num.trim().parse::<i64>().unwrap())
    }
    nums_list.sort();
    return nums_list;
}

fn multiply_sum(nums_list: &Vec<i64>) -> i64 {
    let size: usize = nums_list.len();
    let mut i: usize = 0;
    while i < size - 2 {
        let mut j: usize = size - 1;
        while j > i {
            let res: i64 = nums_list[i] + nums_list[j];
            if res == 2020 {
                return nums_list[i] * nums_list[j];
            } else if res < 2020 {
                break;
            }
            j -= 1;
        }
        i += 1;
    }
    return 0;
}

fn multiply_sum_three(nums_list: &Vec<i64>) -> i64 {
    let size: usize = nums_list.len();
    let mut i: usize = 0;
    while i < size - 2 {
        let mut j: usize = 1;
        while j < size - 1 {
            let mut k: usize = 2;
            while k < size {
                let res: i64 = nums_list[i] + nums_list[j] + nums_list[k];
                if res == 2020 {
                    return nums_list[i] * nums_list[j] * nums_list[k];
                } else if res > 2020 {
                    break;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    return 0;
}
