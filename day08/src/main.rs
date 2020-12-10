use std::fs;

fn main() {
    let mut ops: Vec<(String, i64)> = get_input();
    let acc: i64 = acc_value(&ops);
    println!("acc value: {}", acc);
    let final_acc: i64 = fix_instruction(&mut ops);
    println!("final acc value: {}", final_acc);
}

fn get_input() -> Vec<(String, i64)> {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<&str> = content.split('\n').collect();
    let mut ops: Vec<(String, i64)> = Vec::new();
    for line in lines {
        let spl: Vec<&str> = line.split(' ').map(|s| s.trim()).collect();
        let sign: char = spl[1].chars().nth(0).unwrap();
        let mut num: i64 = spl[1][1..].parse::<i64>().unwrap();
        if sign == '-' {
            num *= -1;
        }
        ops.push((spl[0].to_string(), num));
    }
    ops
}

fn acc_value(ops: &Vec<(String, i64)>) -> i64 {
    let mut acc: i64 = 0;
    let mut i: usize = 0;
    let mut has_been_run: Vec<bool> = vec![false; ops.len()];
    loop {
        if has_been_run[i] {
            break;
        }
        has_been_run[i] = true;
        match ops[i].0.as_ref() {
            "acc" => {
                acc += ops[i].1;
                i += 1;
            }
            "jmp" => i = (i as i64 + ops[i].1) as usize,
            "nop" => i += 1,
            _ => println!("Something fucked up."),
        }
    }
    acc
}

fn fix_instruction(ops: &mut Vec<(String, i64)>) -> i64 {
    let mut i: usize = 0;
    let mut final_acc: i64 = 0;
    while i < ops.len() {
        match ops[i].0.as_ref() {
            "jmp" => {
                ops[i].0 = "nop".to_string();
                let (bif, fa) = is_fixed(&ops);
                if bif {
                    final_acc = fa;
                    break;
                }
                ops[i].0 = "jmp".to_string();
            }
            "nop" => {
                ops[i].0 = "jmp".to_string();
                let (bif, fa) = is_fixed(&ops);
                if bif {
                    final_acc = fa;
                    break;
                }
                ops[i].0 = "nop".to_string();
            }
            _ => (),
        }
        i += 1;
    }
    final_acc
}

fn is_fixed(ops: &Vec<(String, i64)>) -> (bool, i64) {
    let mut is_not_inf: bool = true;
    let mut acc: i64 = 0;
    let mut i: usize = 0;
    let mut has_been_run: Vec<bool> = vec![false; ops.len()];
    while i < ops.len() {
        if has_been_run[i] {
            is_not_inf = false;
            break;
        }
        has_been_run[i] = true;
        match ops[i].0.as_ref() {
            "acc" => {
                acc += ops[i].1;
                i += 1;
            }
            "jmp" => i = (i as i64 + ops[i].1) as usize,
            "nop" => i += 1,
            _ => println!("Something fucked up."),
        }
    }
    (is_not_inf, acc)
}
