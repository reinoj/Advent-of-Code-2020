use std::fs;

fn main() {
    let initial_state: Vec<Vec<char>> = get_input();
    let count: u64 = conway_cubes(&initial_state);
    println!("count: {}", count);
    let hypercount: u64 = conway_hypercubes(&initial_state);
    println!("hyper count: {}", hypercount);
}

fn get_input() -> Vec<Vec<char>> {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<&str> = content.split('\n').map(|s| s.trim()).collect();
    let mut initial_state: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut tmp: Vec<char> = Vec::new();
        for c in line.chars() {
            tmp.push(c);
        }
        initial_state.push(tmp);
    }
    initial_state
}

fn conway_cubes(is: &Vec<Vec<char>>) -> u64 {
    // 6*2 + 1
    let x: usize = 13;
    let isy_len: usize = is.len();
    let y: usize = isy_len + 12;
    let isz_len: usize = is[0].len();
    let z: usize = isz_len + 12;
    let midx: usize = x / 2;
    let midy: usize = y / 2;
    let midz: usize = z / 2;
    let mut prev: Vec<Vec<Vec<char>>> = vec![vec![vec!['.'; z]; y]; x];
    for j in 0..isy_len {
        for k in 0..isz_len {
            prev[midx][j + 6][k + 6] = is[j][k];
        }
    }
    let mut cur: Vec<Vec<Vec<char>>> = prev.clone();

    for n in 1..=6 {
        for i in midx - n..=midx + n {
            for j in midy - isy_len / 2 - n..midy + isy_len / 2 + n {
                for k in midz - isz_len / 2 - n..midz + isz_len / 2 + n {
                    let lowi: usize = if i == 0 { 0 } else { i - 1 };
                    let highi: usize = if i == x - 1 { i } else { i + 1 };
                    let lowj: usize = if j == 0 { 0 } else { j - 1 };
                    let highj: usize = if j == y - 1 { j } else { j + 1 };
                    let lowk: usize = if k == 0 { 0 } else { k - 1 };
                    let highk: usize = if k == z - 1 { k } else { k + 1 };
                    let mut active_count: u64 = 0;
                    for ni in lowi..=highi {
                        for nj in lowj..=highj {
                            for nk in lowk..=highk {
                                if ni != i || nj != j || nk != k {
                                    if prev[ni][nj][nk] == '#' {
                                        active_count += 1;
                                    }
                                }
                            }
                        }
                    }
                    match prev[i][j][k] {
                        '#' => {
                            if active_count != 2 && active_count != 3 {
                                cur[i][j][k] = '.';
                            } else {
                                cur[i][j][k] = '#';
                            }
                        }
                        '.' => {
                            if active_count == 3 {
                                cur[i][j][k] = '#';
                            } else {
                                cur[i][j][k] = '.';
                            }
                        }
                        _ => println!("Character isn't a '#' or '.'."),
                    }
                }
            }
        }
        if n < 6 {
            prev = cur.clone();
        }
    }

    let mut count: u64 = 0;
    for i in cur {
        for j in i {
            for k in j {
                if k == '#' {
                    count += 1;
                }
            }
        }
    }
    count
}

fn conway_hypercubes(is: &Vec<Vec<char>>) -> u64 {
    // 6*2 + 1
    let x: usize = 13;
    let isy_len: usize = is.len();
    let y: usize = isy_len + 12;
    let isz_len: usize = is[0].len();
    let z: usize = isz_len + 12;
    let w: usize = 13;
    let midx: usize = x / 2;
    let midy: usize = y / 2;
    let midz: usize = z / 2;
    let midw: usize = w / 2;
    let mut prev: Vec<Vec<Vec<Vec<char>>>> = vec![vec![vec![vec!['.'; z]; y]; w]; x];
    for j in 0..isy_len {
        for k in 0..isz_len {
            prev[midx][midw][j + 6][k + 6] = is[j][k];
        }
    }
    let mut cur: Vec<Vec<Vec<Vec<char>>>> = prev.clone();

    for n in 1..=6 {
        for i in midx - n..=midx + n {
            for l in midw - n..=midw + n {
                for j in midy - isy_len / 2 - n..midy + isy_len / 2 + n {
                    for k in midz - isz_len / 2 - n..midz + isz_len / 2 + n {
                        let lowi: usize = if i == 0 { 0 } else { i - 1 };
                        let highi: usize = if i == x - 1 { i } else { i + 1 };
                        let lowj: usize = if j == 0 { 0 } else { j - 1 };
                        let highj: usize = if j == y - 1 { j } else { j + 1 };
                        let lowk: usize = if k == 0 { 0 } else { k - 1 };
                        let highk: usize = if k == z - 1 { k } else { k + 1 };
                        let lowl: usize = if l == 0 { 0 } else { l - 1 };
                        let highl: usize = if l == w - 1 { l } else { l + 1 };
                        let mut active_count: u64 = 0;
                        for ni in lowi..=highi {
                            for nl in lowl..=highl {
                                for nj in lowj..=highj {
                                    for nk in lowk..=highk {
                                        if ni != i || nj != j || nk != k || nl != l {
                                            if ni > 12 || nl > 12 {
                                                // println!(
                                                //     "i: {}, ni: {}, l: {}, nl: {}",
                                                //     i, ni, l, nl
                                                // );
                                            }
                                            if prev[ni][nl][nj][nk] == '#' {
                                                active_count += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        match prev[i][l][j][k] {
                            '#' => {
                                if active_count != 2 && active_count != 3 {
                                    cur[i][l][j][k] = '.';
                                } else {
                                    cur[i][l][j][k] = '#';
                                }
                            }
                            '.' => {
                                if active_count == 3 {
                                    cur[i][l][j][k] = '#';
                                } else {
                                    cur[i][l][j][k] = '.';
                                }
                            }
                            _ => println!("Character isn't a '#' or '.'."),
                        }
                    }
                }
            }
        }
        if n < 6 {
            prev = cur.clone();
        }
    }

    let mut count: u64 = 0;
    for i in cur {
        for l in i {
            for j in l {
                for k in j {
                    if k == '#' {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
