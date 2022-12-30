use std::cmp;

pub fn p1(all_lines: &Vec<String>) -> i32 {
    let mut cur_max = 0;
    let mut cur_acc = 0;

    for line in all_lines {
        if line == "" {
            cur_max = cmp::max(cur_max, cur_acc);
            cur_acc = 0;
        } else {
            let this_int: i32 = line.parse().unwrap();
            cur_acc += this_int;
        }
    }

    cur_max
}

pub fn p2(all_lines: &Vec<String>) -> i32 {
    let mut cur_maxes: [i32; 3] = [0, 0, 0];
    let mut cur_acc = 0;

    for line in all_lines {
        if line == "" {
            let min_max = cmp::min(cur_maxes[0], cmp::min(cur_maxes[1], cur_maxes[2]));
            let idx = cur_maxes.iter().position(|&i| i == min_max).unwrap();

            if cur_acc > min_max {
                cur_maxes[idx] = cur_acc;
            }

            cur_acc = 0;
        } else {
            let this_int: i32 = line.parse().unwrap();
            cur_acc += this_int;
        }
    }

    let mut sum = 0;
    for i in 0..cur_maxes.len() {
        sum += cur_maxes[i];
    }

    sum
}
