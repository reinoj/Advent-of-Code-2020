use std::fs;

fn main() {
    let directions: Vec<(char, i64)> = get_input();
    let m_d: i64 = get_manhattan_distance(&directions);
    println!("Manhattan distance: {}", m_d);
    let m_d_new: i64 = new_waypoint_rule(&directions);
    println!("With new rule: {}", m_d_new);
}

fn get_input() -> Vec<(char, i64)> {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<&str> = content.split('\n').map(|s| s.trim()).collect();
    let mut directions: Vec<(char, i64)> = Vec::new();
    for line in lines {
        let c: char = line.chars().nth(0).unwrap();
        let i: i64 = line[1..].parse::<i64>().unwrap();
        directions.push((c, i));
    }
    directions
}

fn get_manhattan_distance(directions: &Vec<(char, i64)>) -> i64 {
    let mut n_s: i64 = 0;
    let mut e_w: i64 = 0;
    let mut facing: char = 'E';
    let mut f_angle: i64 = 90;
    for (c, i) in directions {
        match c {
            'N' | 'S' | 'E' | 'W' => nsew_add(&c, *i, &mut n_s, &mut e_w),
            'F' => nsew_add(&facing, *i, &mut n_s, &mut e_w),
            'L' | 'R' => {
                if *c == 'L' {
                    f_angle -= i;
                } else {
                    f_angle += i;
                }
                if f_angle < 0 {
                    f_angle += 360;
                } else if f_angle >= 360 {
                    f_angle -= 360;
                }
                match f_angle {
                    0 => facing = 'N',
                    90 => facing = 'E',
                    180 => facing = 'S',
                    270 => facing = 'W',
                    _ => println!("Something fucked up."),
                }
            }
            _ => println!("Something fucked up."),
        }
    }
    n_s.abs() + e_w.abs()
}

fn nsew_add(c: &char, i: i64, n_s: &mut i64, e_w: &mut i64) {
    match c {
        'N' => *n_s += i,
        'S' => *n_s -= -i,
        'E' => *e_w += i,
        'W' => *e_w -= -i,
        _ => println!("nsew_add error."),
    }
}

fn new_waypoint_rule(directions: &Vec<(char, i64)>) -> i64 {
    let mut n_s: i64 = 0;
    let mut e_w: i64 = 0;
    let mut waypoint_facing: Vec<char> = vec!['N', 'E'];
    let mut waypoint_f_angle: Vec<i64> = vec![0, 90];
    let mut waypoint: Vec<i64> = vec![1, 10];
    for (c, i) in directions {
        let (ns_ind, ew_ind): (usize, usize) =
            if waypoint_facing[0] == 'N' || waypoint_facing[0] == 'S' {
                (0, 1)
            } else {
                (1, 0)
            };
        match c {
            'N' => waypoint[ns_ind] += *i,
            'S' => waypoint[ns_ind] -= *i,
            'E' => waypoint[ew_ind] += *i,
            'W' => waypoint[ew_ind] -= *i,
            'F' => {
                for j in 0..=1 {
                    nsew_add(&waypoint_facing[j], i * waypoint[j], &mut n_s, &mut e_w);
                }
            }
            'L' | 'R' => {
                if *c == 'L' {
                    waypoint_f_angle = waypoint_f_angle.iter().map(|wfa| *wfa - i).collect();
                } else {
                    waypoint_f_angle = waypoint_f_angle.iter().map(|wfa| *wfa + i).collect();
                }
                let prev_facing: Vec<char> = waypoint_facing.clone();
                for j in 0..=1 {
                    if waypoint_f_angle[j] < 0 {
                        waypoint_f_angle[j] += 360;
                    } else if waypoint_f_angle[j] >= 360 {
                        waypoint_f_angle[j] -= 360;
                    }
                    match waypoint_f_angle[j] {
                        0 => waypoint_facing[j] = 'N',
                        90 => waypoint_facing[j] = 'E',
                        180 => waypoint_facing[j] = 'S',
                        270 => waypoint_facing[j] = 'W',
                        _ => println!("L R waypoint facing error."),
                    }
                }
                for j in 0..=1 {
                    match waypoint_facing[j] {
                        'N' => {
                            if prev_facing[j] == 'S' || prev_facing[j] == 'W' {
                                waypoint[j] *= -1;
                            }
                        }
                        'S' => {
                            if prev_facing[j] == 'N' || prev_facing[j] == 'E' {
                                waypoint[j] *= -1;
                            }
                        }
                        'E' => {
                            if prev_facing[j] == 'W' || prev_facing[j] == 'S' {
                                waypoint[j] *= -1;
                            }
                        }
                        'W' => {
                            if prev_facing[j] == 'E' || prev_facing[j] == 'N' {
                                waypoint[j] *= -1;
                            }
                        }
                        _ => println!("negate waypoint error."),
                    }
                }
            }
            _ => println!("main match error."),
        }
    }
    n_s.abs() + e_w.abs()
}
