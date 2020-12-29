use regex::Regex;
use std::fs;

fn main() {
    let equations: Vec<Vec<char>> = get_input();
    let sum: u64 = sum_of_expressions(&equations);
    println!("sum: {}", sum);
    let mut sequations: Vec<String> = get_input_strings();
    let ssum: u64 = with_precedence(&mut sequations);
    println!("2nd sum: {}", ssum);
}

fn get_input() -> Vec<Vec<char>> {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<String> = content.split('\n').map(|s| s.trim().to_string()).collect();
    let mut equations: Vec<Vec<char>> = Vec::new();
    for mut line in lines {
        line.retain(|c| c != ' ');
        equations.push(line.chars().collect());
    }
    equations
}

fn get_input_strings() -> Vec<String> {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<String> = content.split('\n').map(|s| s.trim().to_string()).collect();
    let mut equations: Vec<String> = Vec::new();
    for mut line in lines {
        line.retain(|c| c != ' ');
        equations.push(line);
    }
    equations
}

fn sum_of_expressions(equations: &Vec<Vec<char>>) -> u64 {
    let mut sum: u64 = 0;
    for eq in equations {
        let mut totals: Vec<u64> = vec![0];
        let mut exprs: Vec<char> = vec!['+'];
        let mut parentheses_count: usize = 0;
        for c in eq {
            match c {
                '(' => {
                    parentheses_count += 1;
                    totals.push(0);
                    exprs.push('+');
                }
                ')' => {
                    match exprs[parentheses_count - 1] {
                        '+' => totals[parentheses_count - 1] += totals[parentheses_count],
                        '*' => totals[parentheses_count - 1] *= totals[parentheses_count],
                        _ => println!("character isn't '+' or '*'."),
                    }
                    totals.remove(parentheses_count);
                    exprs.remove(parentheses_count);
                    parentheses_count -= 1;
                }
                '+' | '*' => exprs[parentheses_count] = *c,
                _ => {
                    if c.is_ascii_digit() {
                        match exprs[parentheses_count] {
                            '+' => totals[parentheses_count] += c.to_digit(10).unwrap() as u64,
                            '*' => totals[parentheses_count] *= c.to_digit(10).unwrap() as u64,
                            _ => println!("character isn't '+' or '*'."),
                        }
                    } else {
                        println!("Unaccounted for char: {}", c);
                    }
                }
            }
        }
        sum += totals[0];
    }
    sum
}

fn with_precedence(equations: &mut Vec<String>) -> u64 {
    let mut sum: u64 = 0;
    let parentheses_re = Regex::new(r"\([\d\+\*]*\)").unwrap();
    let add_re = Regex::new(r"\d*\+\d*").unwrap();
    let mult_re = Regex::new(r"\d*\*\d*").unwrap();
    for i in 0..equations.len() {
        loop {
            match parentheses_re.find(&equations[i]) {
                Some(m) => {
                    let tmp_num: u64 = evaluate_string(
                        &equations[i][m.start() + 1..m.end() - 1].to_string(),
                        &add_re,
                        &mult_re,
                    );
                    let mut tmp_str: String = String::new();
                    tmp_str.push_str(&equations[i][0..m.start()]);
                    tmp_str.push_str(tmp_num.to_string().as_str());
                    tmp_str.push_str(&equations[i][m.end()..]);
                    equations[i].clear();
                    equations[i] = tmp_str;
                }
                None => match add_re.find(equations[i].as_str()) {
                    Some(m) => {
                        let nums: Vec<u64> = equations[i][m.start()..m.end()]
                            .split('+')
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect();
                        let mut tmp: String = String::new();
                        tmp.push_str(&equations[i][0..m.start()]);
                        tmp.push_str((nums[0] + nums[1]).to_string().as_str());
                        tmp.push_str(&equations[i][m.end()..]);
                        equations[i].clear();
                        equations[i] = tmp;
                    }
                    None => match mult_re.find(equations[i].as_str()) {
                        Some(m) => {
                            let nums: Vec<u64> = equations[i][m.start()..m.end()]
                                .split('*')
                                .map(|s| s.parse::<u64>().unwrap())
                                .collect();
                            let mut tmp: String = String::new();
                            tmp.push_str(&equations[i][0..m.start()]);
                            tmp.push_str((nums[0] * nums[1]).to_string().as_str());
                            tmp.push_str(&equations[i][m.end()..]);
                            equations[i].clear();
                            equations[i] = tmp;
                        }
                        None => break,
                    },
                },
            }
        }
        sum += equations[i].parse::<u64>().unwrap();
    }
    sum
}

fn evaluate_string(eq: &str, add_re: &regex::Regex, mult_re: &regex::Regex) -> u64 {
    let mut eq_str: String = eq.to_string().clone();
    loop {
        match add_re.find(eq_str.as_str()) {
            Some(m) => {
                let nums: Vec<u64> = eq_str[m.start()..m.end()]
                    .split('+')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
                let mut tmp: String = String::new();
                tmp.push_str(&eq_str[0..m.start()]);
                tmp.push_str((nums[0] + nums[1]).to_string().as_str());
                tmp.push_str(&eq_str[m.end()..]);
                eq_str.clear();
                eq_str = tmp;
            }
            None => match mult_re.find(eq_str.as_str()) {
                Some(m) => {
                    let nums: Vec<u64> = eq_str[m.start()..m.end()]
                        .split('*')
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect();
                    let mut tmp: String = String::new();
                    tmp.push_str(&eq_str[0..m.start()]);
                    tmp.push_str((nums[0] * nums[1]).to_string().as_str());
                    tmp.push_str(&eq_str[m.end()..]);
                    eq_str.clear();
                    eq_str = tmp;
                }
                None => break,
            },
        }
    }
    eq_str.parse::<u64>().unwrap()
}
