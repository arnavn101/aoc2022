use std::collections::{HashSet, VecDeque};

pub fn p1(lines: &Vec<String>) -> i32 {
    let mut x = 1;
    let mut s = 0;

    let cycles = HashSet::from([20, 60, 100, 140, 180, 220]);
    let mut tasks: VecDeque<[i32; 2]> = VecDeque::new();

    for c in 1..221 {
        let t = tasks.len();
        if t > 0 {
            if tasks[t - 1][0] == 1 {
                x += tasks.pop_back().unwrap()[1];
            } else {
                tasks[t - 1][0] -= 1;
            }
        }

        if cycles.contains(&(c)) {
            s += (c as i32) * x;
        }

        let i = c - 1;
        if i < lines.len() {
            let split = lines[i].split(" ");
            let items: Vec<&str> = split.collect();
            if items[0] == "addx" {
                let mag: i32 = items[1].parse().unwrap();
                tasks.push_front([2, mag]);
            } else {
                tasks.push_front([1, 0]);
            }
        }
    }

    s
}

pub fn p2(lines: &Vec<String>) -> u32 {
    let mut x = 1;

    let mut crt = [[" "; 40]; 6];
    let mut tasks: VecDeque<[i32; 2]> = VecDeque::new();

    for c in 1..241 {
        let t = tasks.len();
        if t > 0 {
            if tasks[t - 1][0] == 1 {
                x += tasks.pop_back().unwrap()[1];
            } else {
                tasks[t - 1][0] -= 1;
            }
        }

        let r: usize = (c - 1) / 40;
        let r_pos: usize = (c - 1) % 40;
        let x_pos = HashSet::from([x - 1, x, x + 1]);

        if x_pos.contains(&(r_pos as i32)) {
            crt[r][r_pos] = "█";
        }

        let i = c - 1;
        if i < lines.len() {
            let split = lines[i].split(" ");
            let items: Vec<&str> = split.collect();
            if items[0] == "addx" {
                let mag: i32 = items[1].parse().unwrap();
                tasks.push_front([2, mag]);
            } else {
                tasks.push_front([1, 0]);
            }
        }
    }

    for row in crt {
        println!("{:?}", row.join(""));
    }

    1
}
