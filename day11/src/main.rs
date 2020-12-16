use std::convert::TryFrom;
use std::fs;

fn main() {
    let seats: Vec<Vec<char>> = get_input();
    let occupied_seats: u64 = how_many_occupied_seats(&seats);
    println!("Occupied Seats: {}", occupied_seats);
    let occupied_seats2: u64 = how_many_occupied_seats2(&seats);
    println!("New Rule: {}", occupied_seats2);
}

fn get_input() -> Vec<Vec<char>> {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<&str> = content.split('\n').map(|s| s.trim()).collect();
    let mut seats: Vec<Vec<char>> = Vec::new();
    for line in &lines {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        seats.push(row);
    }
    seats
}

fn how_many_occupied_seats(seats: &Vec<Vec<char>>) -> u64 {
    let mut prev: Vec<Vec<char>> = seats.clone();
    let mut cur: Vec<Vec<char>> = Vec::new();
    let i_len: usize = seats.len();
    let j_len: usize = seats[0].len();
    loop {
        let mut is_same: bool = true;
        for i in 0..i_len {
            let mut cur_line: Vec<char> = Vec::new();
            for j in 0..j_len {
                match prev[i][j] {
                    'L' => {
                        let mut empty_count: u8 = 0;
                        let lowi: usize = if i == 0 {
                            empty_count += 3;
                            i
                        } else {
                            i - 1
                        };
                        let highi: usize = if i == i_len - 1 {
                            empty_count += 3;
                            i
                        } else {
                            i + 1
                        };
                        for si in lowi..=highi {
                            let lowj: usize = if j == 0 {
                                empty_count += 1;
                                j
                            } else {
                                j - 1
                            };
                            let highj: usize = if j == j_len - 1 {
                                empty_count += 1;
                                j
                            } else {
                                j + 1
                            };
                            for sj in lowj..=highj {
                                if i != si || j != sj {
                                    if prev[si][sj] == 'L' || prev[si][sj] == '.' {
                                        empty_count += 1;
                                    }
                                }
                            }
                        }
                        if empty_count == 8 {
                            cur_line.push('#');
                        } else {
                            cur_line.push('L');
                        }
                    }
                    '#' => {
                        let mut occupied_count: u8 = 0;
                        let lowi: usize = if i == 0 { i } else { i - 1 };
                        let highi: usize = if i == i_len - 1 { i } else { i + 1 };
                        for si in lowi..=highi {
                            let lowj: usize = if j == 0 { j } else { j - 1 };
                            let highj: usize = if j == j_len - 1 { j } else { j + 1 };
                            for sj in lowj..=highj {
                                if i != si || j != sj {
                                    if prev[si][sj] == '#' {
                                        occupied_count += 1;
                                    }
                                }
                            }
                        }
                        if occupied_count >= 4 {
                            cur_line.push('L');
                        } else {
                            cur_line.push('#');
                        }
                    }
                    '.' => cur_line.push('.'),
                    _ => println!("Something fucked up."),
                }
            }
            cur.push(cur_line);
            for j in 0..j_len {
                is_same = is_same && (prev[i][j] == cur[i][j]);
            }
        }
        if is_same {
            break;
        } else {
            prev = cur;
            cur = Vec::new();
        }
    }
    let mut occupied_seats: u64 = 0;
    for row in cur {
        for c in row {
            if c == '#' {
                occupied_seats += 1;
            }
        }
    }
    occupied_seats
}

fn how_many_occupied_seats2(seats: &Vec<Vec<char>>) -> u64 {
    let mut prev: Vec<Vec<char>> = seats.clone();
    let mut cur: Vec<Vec<char>> = Vec::new();
    let i_len: usize = seats.len();
    let j_len: usize = seats[0].len();
    loop {
        let mut is_same: bool = true;
        for i in 0..i_len {
            let mut cur_line: Vec<char> = Vec::new();
            for j in 0..j_len {
                match prev[i][j] {
                    'L' => {
                        let occupied_count: u8 =
                            count_occupied_seats(&prev, &i, &j, &i_len, &j_len);
                        if occupied_count == 0 {
                            cur_line.push('#');
                        } else {
                            cur_line.push('L');
                        }
                    }
                    '#' => {
                        let occupied_count: u8 =
                            count_occupied_seats(&prev, &i, &j, &i_len, &j_len);
                        if occupied_count >= 5 {
                            cur_line.push('L');
                        } else {
                            cur_line.push('#');
                        }
                    }
                    '.' => cur_line.push('.'),
                    _ => println!("Something fucked up."),
                }
            }
            cur.push(cur_line);
            for j in 0..j_len {
                is_same = is_same && (prev[i][j] == cur[i][j]);
            }
        }
        if is_same {
            break;
        } else {
            prev = cur;
            cur = Vec::new();
        }
    }
    let mut occupied_seats: u64 = 0;
    for row in cur {
        for c in row {
            if c == '#' {
                occupied_seats += 1;
            }
        }
    }
    occupied_seats
}

fn count_occupied_seats(
    prev: &Vec<Vec<char>>,
    i: &usize,
    j: &usize,
    i_len: &usize,
    j_len: &usize,
) -> u8 {
    let mut occupied_count: u8 = 0;
    let mut counted: Vec<bool> = vec![false; 8];
    let mut diff: i64 = 1;
    let ni_len: i64 = i64::try_from(*i_len).unwrap();
    let nj_len: i64 = i64::try_from(*j_len).unwrap();
    while counted.iter().fold(false, |acc, &b| acc || !b) {
        for ind in 0..8 {
            if !counted[ind] {
                let mut ni: usize = usize::MAX;
                let mut nj: usize = usize::MAX;
                match ind {
                    0 => {
                        let ni64: i64 = i64::try_from(*i).unwrap() - diff;
                        if ni64 >= 0 {
                            ni = usize::try_from(ni64).unwrap();
                            nj = *j;
                        } else {
                            counted[ind] = true;
                            continue;
                        }
                    }
                    1 => {
                        let ni64: i64 = i64::try_from(*i).unwrap() - diff;
                        let nj64: i64 = i64::try_from(*j).unwrap() + diff;
                        if ni64 >= 0 && nj64 < nj_len {
                            ni = usize::try_from(ni64).unwrap();
                            nj = usize::try_from(nj64).unwrap();
                        } else {
                            counted[ind] = true;
                            continue;
                        }
                    }
                    2 => {
                        let nj64: i64 = i64::try_from(*j).unwrap() + diff;
                        if nj64 < nj_len {
                            ni = *i;
                            nj = usize::try_from(nj64).unwrap();
                        } else {
                            counted[ind] = true;
                            continue;
                        }
                    }
                    3 => {
                        let ni64: i64 = i64::try_from(*i).unwrap() + diff;
                        let nj64: i64 = i64::try_from(*j).unwrap() + diff;
                        if ni64 < ni_len && nj64 < nj_len {
                            ni = usize::try_from(ni64).unwrap();
                            nj = usize::try_from(nj64).unwrap();
                        } else {
                            counted[ind] = true;
                            continue;
                        }
                    }
                    4 => {
                        let ni64: i64 = i64::try_from(*i).unwrap() + diff;
                        if ni64 < ni_len {
                            ni = usize::try_from(ni64).unwrap();
                            nj = *j;
                        } else {
                            counted[ind] = true;
                            continue;
                        }
                    }
                    5 => {
                        let ni64: i64 = i64::try_from(*i).unwrap() + diff;
                        let nj64: i64 = i64::try_from(*j).unwrap() - diff;
                        if ni64 < ni_len && nj64 >= 0 {
                            ni = usize::try_from(ni64).unwrap();
                            nj = usize::try_from(nj64).unwrap();
                        } else {
                            counted[ind] = true;
                            continue;
                        }
                    }
                    6 => {
                        let nj64: i64 = i64::try_from(*j).unwrap() - diff;
                        if nj64 >= 0 {
                            ni = *i;
                            nj = usize::try_from(nj64).unwrap();
                        } else {
                            counted[ind] = true;
                            continue;
                        }
                    }
                    7 => {
                        let ni64: i64 = i64::try_from(*i).unwrap() - diff;
                        let nj64: i64 = i64::try_from(*j).unwrap() - diff;
                        if ni64 >= 0 && nj64 >= 0 {
                            ni = usize::try_from(ni64).unwrap();
                            nj = usize::try_from(nj64).unwrap();
                        } else {
                            counted[ind] = true;
                            continue;
                        }
                    }
                    _ => {
                        println!("Something fucked up.");
                        continue;
                    }
                }
                match prev[ni][nj] {
                    'L' => counted[ind] = true,
                    '#' => {
                        occupied_count += 1;
                        counted[ind] = true
                    }
                    _ => {}
                }
            }
        }
        diff += 1;
    }
    occupied_count
}
