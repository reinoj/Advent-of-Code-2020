use std::fs;

fn main() {
    let passports: Vec<Vec<String>> = get_input();
    let count: i32 = count_valid(&passports);
    let valid_count: i32 = count_valid2(&passports);
    println!("count: {}\nvalid count: {}", count, valid_count);
}

fn get_input() -> Vec<Vec<String>> {
    let content = fs::read_to_string("src/input").expect("Error while reading file.");
    let split_lines: Vec<&str> = content.split(|c| c == '\n' || c == ' ').collect();
    let mut passports: Vec<Vec<String>> = Vec::new();
    let mut passport: Vec<String> = Vec::new();
    for line in split_lines {
        if line.len() <= 1 {
            passports.push(passport.clone());
            passport = Vec::new();
        } else {
            passport.push(line.trim().to_string());
        }
    }
    passports
}

fn count_valid(passports: &Vec<Vec<String>>) -> i32 {
    let mut count: i32 = 0;
    for passport in passports {
        if passport.len() < 7 {
            continue;
        } else {
            let mut field_check: Vec<bool> = vec![false; 7];
            for field in passport {
                let kv: Vec<&str> = field.split(':').collect();
                match kv[0] {
                    "byr" => field_check[0] = true,
                    "iyr" => field_check[1] = true,
                    "eyr" => field_check[2] = true,
                    "hgt" => field_check[3] = true,
                    "hcl" => field_check[4] = true,
                    "ecl" => field_check[5] = true,
                    "pid" => field_check[6] = true,
                    _ => continue,
                }
            }
            let mut is_valid: bool = true;
            for b in field_check {
                is_valid = is_valid && b;
                if !b {
                    break;
                }
            }
            if is_valid {
                count += 1;
            }
        }
    }

    count
}

fn count_valid2(passports: &Vec<Vec<String>>) -> i32 {
    let mut count: i32 = 0;
    let eye_colors: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    for passport in passports {
        if passport.len() < 7 {
            continue;
        } else {
            let mut field_check: Vec<bool> = vec![false; 7];
            for field in passport {
                let kv: Vec<&str> = field.split(':').collect();
                match kv[0] {
                    "byr" => {
                        let byr: i32 = kv[1].parse::<i32>().unwrap();
                        if byr >= 1920 && byr <= 2002 {
                            field_check[0] = true;
                        }
                    }
                    "iyr" => {
                        let iyr: i32 = kv[1].parse::<i32>().unwrap();
                        if iyr >= 2010 && iyr <= 2020 {
                            field_check[1] = true;
                        }
                    }
                    "eyr" => {
                        let eyr: i32 = kv[1].parse::<i32>().unwrap();
                        if eyr >= 2020 && eyr <= 2030 {
                            field_check[2] = true;
                        }
                    }
                    "hgt" => {
                        let unit_in: Option<usize> = kv[1].find("in");
                        match unit_in {
                            Some(num) => {
                                let hgt: i32 = kv[1][..num].parse::<i32>().unwrap();
                                if hgt >= 59 && hgt <= 76 {
                                    field_check[3] = true;
                                }
                            }
                            None => {
                                let unit_cm: Option<usize> = kv[1].find("cm");
                                match unit_cm {
                                    Some(num) => {
                                        let hgt: i32 = kv[1][..num].parse::<i32>().unwrap();
                                        if hgt >= 150 && hgt <= 193 {
                                            field_check[3] = true;
                                        }
                                    }
                                    None => continue,
                                }
                            }
                        }
                    }
                    "hcl" => {
                        if kv[1].chars().nth(0).unwrap() == '#' {
                            for c in kv[1][1..].chars() {
                                if !c.is_digit(16) {
                                    continue;
                                }
                            }
                            field_check[4] = true;
                        }
                    }
                    "ecl" => {
                        if eye_colors.iter().any(|&s| s == kv[1]) {
                            field_check[5] = true;
                        }
                    }
                    "pid" => {
                        if kv[1].len() == 9 {
                            if kv[1].chars().any(|c| c.is_digit(10)) {
                                field_check[6] = true;
                            }
                        }
                    }
                    _ => continue,
                }
            }
            let mut is_valid: bool = true;
            for b in field_check {
                is_valid = is_valid && b;
                if !b {
                    break;
                }
            }
            if is_valid {
                count += 1;
            }
        }
    }

    count
}
