use kdam::tqdm;
use std::cmp::min;
use std::collections::HashMap;

pub fn p1(lines: &Vec<String>) -> u32 {
    let bps = parse_input(lines);
    bps.iter()
        .map(|bp| simulate_bp(bp, 24))
        .enumerate()
        .fold(0, |acc, (idx, e)| acc + e * (idx + 1) as u32)
}

pub fn p2(lines: &Vec<String>) -> u32 {
    let bps = parse_input(lines);
    let three_bps = bps[..min(bps.len(), 3)].to_vec();
    three_bps
        .iter()
        .map(|bp| simulate_bp(bp, 32))
        .fold(1, |acc, e| acc * e)
}

fn simulate_bp(bp: &Vec<Vec<u32>>, time: u32) -> u32 {
    let mut cur_list: Vec<([u32; 4], [u32; 4])> = Vec::from([([0, 0, 0, 0], [1, 0, 0, 0])]);

    for _ in tqdm!(0..time) {
        let mut tmp_list: Vec<([u32; 4], [u32; 4])> = Vec::new();
        for (resources, robots) in cur_list.iter() {
            for (robot_idx, robot_resources) in bp.iter().enumerate() {
                let mut robot_cost = [0, 0, 0, 0];
                let mut robot_res = [0, 0, 0, 0];

                if robot_idx <= 3 {
                    robot_res[robot_idx] += 1;
                }

                match robot_idx {
                    0 => robot_cost[0] = robot_resources[0],
                    1 => robot_cost[0] = robot_resources[0],
                    2 => {
                        robot_cost[0] = robot_resources[0];
                        robot_cost[1] = robot_resources[1]
                    }
                    3 => {
                        robot_cost[0] = robot_resources[0];
                        robot_cost[2] = robot_resources[1]
                    }
                    4 => (),
                    _ => panic!(),
                }

                if robot_cost
                    .iter()
                    .enumerate()
                    .all(|(i, &p)| p <= resources[i])
                {
                    let inter_res = sum_arrays(&resources, &robots, 1);
                    let f_resources = sum_arrays(&inter_res, &robot_cost, -1);
                    let f_robots = sum_arrays(&robots, &robot_res, 1);
                    tmp_list.push((f_resources, f_robots));
                }
            }
        }
        cur_list = prune(&tmp_list);
    }

    cur_list.iter().max_by_key(|arr| arr.0[3]).unwrap().0[3]
}

fn prune(res_robots: &Vec<([u32; 4], [u32; 4])>) -> Vec<([u32; 4], [u32; 4])> {
    let mut ht: HashMap<[u32; 8], ([u32; 4], [u32; 4])> = HashMap::new();

    for (a, b) in res_robots {
        let mut k1 = sum_arrays(&a, &b, 1);
        k1.reverse();
        let key = [k1[0], k1[1], k1[2], k1[3], b[3], b[2], b[1], b[0]];
        if !ht.contains_key(&key) {
            ht.insert(key, (*a, *b));
        }
    }

    let mut vals: Vec<([u32; 4], [u32; 4])> = ht.values().map(|v| *v).collect();
    vals.sort_by_key(|(a, b)| {
        let mut k = sum_arrays(&a, &b, 1);
        k.reverse();
        [k[0], k[1], k[2], k[3], b[3], b[2], b[1], b[0]]
    });
    vals.reverse();

    let end_idx = min(1000, vals.len());
    vals[..end_idx].to_vec()
}

fn sum_arrays(arr1: &[u32; 4], arr2: &[u32; 4], c: i32) -> [u32; 4] {
    let mut arr3 = [0; 4];

    for (i, elem) in arr1.iter().enumerate() {
        if c == -1 {
            arr3[i] += elem - arr2[i];
        } else {
            arr3[i] += elem + arr2[i];
        }
    }

    arr3
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<Vec<u32>>> {
    let mut all_bps: Vec<Vec<Vec<u32>>> = Vec::new();

    for line in lines {
        let bp_split = split_by(line, ":");
        let bp_string = bp_split[1].to_string();
        let rb_split = split_by(&bp_string, ".");

        let mut bp: Vec<Vec<u32>> = Vec::new();

        for rb in rb_split.iter() {
            if rb.len() == 0 {
                continue;
            }

            let mut v: Vec<u32> = Vec::new();
            let rb_string = rb.to_string();
            for s in split_by(&rb_string, " ") {
                match s.parse() {
                    Ok(s_num) => v.push(s_num),
                    Err(_) => (),
                }
            }
            bp.push(v);
        }

        bp.push(Vec::from([0]));
        all_bps.push(bp);
    }

    all_bps
}

fn split_by<'a>(line: &'a String, delim: &str) -> Vec<&'a str> {
    let split = line.split(delim);
    split.collect()
}
