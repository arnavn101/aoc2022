use std::collections::VecDeque;

pub fn p1(lines: &Vec<String>) -> String {
    let num_crates: usize = 9;
    let mut all_crates: Vec<VecDeque<char>> = vec![VecDeque::new(); num_crates];
    let mut done_init = false;

    for line in lines.iter() {
        if line.len() == 0 {
            done_init = true;
            continue;
        }

        if !done_init {
            for (i, ch) in line.chars().enumerate() {
                if ch.is_alphabetic() {
                    let idx: usize = ((i / 4) as f32).floor() as usize;
                    all_crates[idx].push_back(ch.to_owned());
                }
            }
        } else {
            let split = line.split(" ");
            let tasks: Vec<&str> = split.collect();

            let its: usize = get_idx(&tasks, 1);
            let from: usize = get_idx(&tasks, 3);
            let to: usize = get_idx(&tasks, 5);

            for _ in 0..its {
                let popped = all_crates[from - 1].pop_front().unwrap();
                all_crates[to - 1].push_front(popped);
            }
        }
    }

    let mut ret_string = String::new();

    for i in 0..num_crates {
        let ch = all_crates[i].front().unwrap().clone();
        ret_string.push(ch);
    }

    ret_string
}

pub fn p2(lines: &Vec<String>) -> String {
    let num_crates: usize = 9;
    let mut all_crates: Vec<VecDeque<char>> = vec![VecDeque::new(); num_crates];
    let mut done_init = false;

    for line in lines.iter() {
        if line.len() == 0 {
            done_init = true;
            continue;
        }

        if !done_init {
            for (i, ch) in line.chars().enumerate() {
                if ch.is_alphabetic() {
                    let idx: usize = ((i / 4) as f32).floor() as usize;
                    all_crates[idx].push_back(ch.to_owned());
                }
            }
        } else {
            let split = line.split(" ");
            let tasks: Vec<&str> = split.collect();

            let its: usize = get_idx(&tasks, 1);
            let from: usize = get_idx(&tasks, 3);
            let to: usize = get_idx(&tasks, 5);

            let mut tmp_arr: Vec<char> = Vec::new();
            for _ in 0..its {
                let popped = all_crates[from - 1].pop_front().unwrap();
                tmp_arr.push(popped);
            }

            for elem in tmp_arr.iter().rev() {
                all_crates[to - 1].push_front(*elem);
            }
        }
    }

    let mut ret_string = String::new();

    for i in 0..num_crates {
        let ch = all_crates[i].front().unwrap().clone();
        ret_string.push(ch);
    }

    ret_string
}

fn get_idx(tasks: &Vec<&str>, i: usize) -> usize {
    return tasks.get(i).unwrap().parse::<i32>().unwrap() as usize;
}
