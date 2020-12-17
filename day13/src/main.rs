use std::fs;

fn main() {
    let time_and_buses: (usize, Vec<usize>) = get_input();
    let val: usize = id_times_wait(&time_and_buses);
    println!("ID * wait time = {}", val);
    let bus_times: Vec<(usize, usize)> = get_input2();
    let timestamp: usize = earliest_timestamp(&bus_times);
    println!("Time: {}", timestamp);
}

fn get_input() -> (usize, Vec<usize>) {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<&str> = content.split('\n').map(|s| s.trim()).collect();
    let mut time_and_buses: (usize, Vec<usize>) = (0, Vec::new());
    time_and_buses.0 = lines[0].parse::<usize>().unwrap();
    let buses: Vec<&str> = lines[1].split(',').map(|s| s.trim()).collect();
    for bus in buses {
        if bus != "x" {
            time_and_buses.1.push(bus.parse::<usize>().unwrap());
        }
    }
    time_and_buses
}

fn id_times_wait(time_and_buses: &(usize, Vec<usize>)) -> usize {
    let mut closest_times: Vec<usize> = Vec::new();
    for bus in &time_and_buses.1 {
        let mut cur: usize = 0;
        while cur < time_and_buses.0 {
            cur += bus;
        }
        closest_times.push(cur);
    }
    let mut min_ind: usize = 0;
    let mut min_diff: usize = usize::MAX;
    for i in 0..time_and_buses.1.len() {
        let dif = closest_times[i] - time_and_buses.0;
        if dif < min_diff {
            min_ind = i;
            min_diff = dif;
        }
    }
    time_and_buses.1[min_ind] * min_diff
}

fn get_input2() -> Vec<(usize, usize)> {
    let content: String = fs::read_to_string("src/input").expect("Error reading input.");
    let lines: Vec<&str> = content.split('\n').map(|s| s.trim()).collect();
    let buses: Vec<&str> = lines[1].split(',').map(|s| s.trim()).collect();
    let mut bus_times: Vec<(usize, usize)> = Vec::new();
    let mut offset: usize = 0;
    for bus in buses {
        if bus != "x" {
            bus_times.push((bus.parse::<usize>().unwrap(), offset));
        }
        offset += 1;
    }
    bus_times
}

// fn chinese_remainder_theorem(bus_times: &Vec<(usize, usize)>) -> usize {
//     let modulo: usize = bus_times.iter().fold(1, |acc, v| acc * v.0);
//     let mut acc: usize = 0;
//     println!("mod: {}", modulo);
//     for i in 0..bus_times.len() {
//         let m: usize = modulo / bus_times[i].0;
//         let y: usize = m % bus_times[i].0;
//         acc += bus_times[i].0 * m * y;
//         println!("m: {}, y: {}, acc: {}", m, y, acc);
//     }
//     acc
// }

fn earliest_timestamp(bus_times: &Vec<(usize, usize)>) -> usize {
    let mut cur_time: usize = bus_times[0].0;
    let mut jump = bus_times[0].0;
    let mut i: usize = 1;
    while i < bus_times.len() {
        if (cur_time + bus_times[i].1) % bus_times[i].0 == 0 {
            jump *= bus_times[i].0;
            i += 1;
        } else {
            cur_time += jump;
        }
    }

    cur_time
}
