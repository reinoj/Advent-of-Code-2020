use std::fs;

static ACROSS: usize = 31;
static DOWN: usize = 323;

fn main() {
    let bool_map: Vec<Vec<bool>> = get_input();
    let encounters: i64 = count_encounters(&bool_map);
    let all_encounters: i64 = count_all_encounters(&bool_map);
    println!(
        "encounters: {}\nall encounters: {}",
        encounters, all_encounters
    );
}

fn get_input() -> Vec<Vec<bool>> {
    let content = fs::read_to_string("src/input.txt").expect("Error while reading file.");
    let line_vec: Vec<&str> = content.split('\n').collect();
    let mut bool_map: Vec<Vec<bool>> = vec![vec![false; ACROSS]; DOWN];
    let mut i: usize = 0;
    while i < DOWN {
        let mut j: usize = 0;
        while j < ACROSS {
            let line: Vec<char> = line_vec[i].chars().collect();
            if line[j] == '#' {
                bool_map[i][j] = true;
            }
            j += 1;
        }
        i += 1;
    }
    bool_map
}

fn count_encounters(bool_map: &Vec<Vec<bool>>) -> i64 {
    let mut encounters: i64 = 0;
    let (mut x, mut y) = (0 as usize, 0 as usize);
    while y < DOWN {
        // println!("x: {}, y: {}", x, y);
        if bool_map[y][x] {
            encounters += 1;
        }
        y += 1;
        x += 3;
        if x >= ACROSS {
            x -= ACROSS;
        }
    }
    encounters
}

fn count_all_encounters(bool_map: &Vec<Vec<bool>>) -> i64 {
    let mut all_encounters: i64 = 1;
    let rights: Vec<usize> = vec![1, 3, 5, 7, 1];
    let downs: Vec<usize> = vec![1, 1, 1, 1, 2];
    let mut i: usize = 0;
    while i < rights.len() {
        let mut encounters: i64 = 0;
        let (mut x, mut y) = (0 as usize, 0 as usize);
        while y < DOWN {
            // println!("x: {}, y: {}", x, y);
            if bool_map[y][x] {
                encounters += 1;
            }
            y += downs[i];
            x += rights[i];
            if x >= ACROSS {
                x -= ACROSS;
            }
        }
        all_encounters *= encounters;
        i += 1;
    }
    all_encounters
}
