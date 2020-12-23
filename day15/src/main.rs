use std::collections::HashMap;

fn main() {
    // let sequence: Vec<u64> = vec![0, 3, 6];
    let sequence: Vec<u64> = vec![10, 16, 6, 0, 1, 17];
    let num_spoken: u64 = find_nth(&sequence, 2020);
    println!("2020th number spoken: {}", num_spoken);
    let num_spoken: u64 = find_nth(&sequence, 30_000_000);
    println!("30,000,000th number spoken: {}", num_spoken);
}

fn find_nth(sequence: &Vec<u64>, n: u64) -> u64 {
    let mut turn: u64 = 1;
    let mut num_hash: HashMap<u64, Vec<u64>> = HashMap::new();
    for i in 0..sequence.len() {
        num_hash.insert(sequence[i], vec![turn, 0, 1]);
        turn += 1;
    }

    let mut prev_num: u64 = sequence[sequence.len() - 1];
    while turn <= n {
        let turn_timesspoken: &Vec<u64> = num_hash.get(&prev_num).unwrap();
        if turn_timesspoken[2] == 1 {
            prev_num = 0;
        } else {
            prev_num = turn_timesspoken[0] - turn_timesspoken[1];
        }
        match num_hash.get(&prev_num) {
            None => {
                num_hash.insert(prev_num, vec![turn, 0, 1]);
            }
            Some(ntt) => {
                *num_hash.entry(prev_num).or_insert(vec![turn, 0, 1]) =
                    vec![turn, ntt[0], ntt[2] + 1];
            }
        }
        turn += 1;
    }
    prev_num
}
