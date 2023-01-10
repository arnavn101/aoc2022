use std::cmp::max;
use std::collections::{HashMap, HashSet};

pub fn p1(lines: &Vec<String>) -> usize {
    get_flow(lines, 31).0
}

pub fn p2(lines: &Vec<String>) -> usize {
    let (_, mut all_ways, init_opened) = get_flow(lines, 27);
    let mut max_flow = 0;
    all_ways.sort_by_key(|(v1, _)| *v1);

    let parsed_ways: Vec<(&usize, HashSet<usize>)> = all_ways
        .iter()
        .map(|(v1, v2)| {
            (
                v1,
                v2.iter().enumerate().fold(HashSet::new(), |mut acc, p| {
                    if !init_opened.contains(&p.0) && *p.1 {
                        acc.insert(p.0);
                        acc
                    } else {
                        acc
                    }
                }),
            )
        })
        .rev()
        .collect();

    for (i, (flow, set)) in parsed_ways.iter().enumerate() {
        let cur_flow = **flow;
        if cur_flow * 2 <= max_flow {
            break;
        }

        for (n_flow, n_set) in parsed_ways.iter().skip(i) {
            let next_flow = **n_flow;

            match set.intersection(n_set).next() {
                Some(_) => (),
                None => max_flow = max(max_flow, cur_flow + next_flow),
            }
        }
    }

    max_flow
}

fn get_flow(
    lines: &Vec<String>,
    req_time: usize,
) -> (usize, Vec<(usize, Vec<bool>)>, HashSet<usize>) {
    let (flow, path) = parse_input(lines);
    let (ht, dist) = get_dist(&path);
    let inv_ht: HashMap<&usize, &&str> = ht.iter().map(|(k, v)| (v, k)).collect();

    let mut opened = vec![false; dist.len()];
    let mut init_opened: HashSet<usize> = HashSet::new();
    let all_zeros = flow.iter().filter(|(_, &e)| e == 0);
    let mut cnt_zeros = 0;

    for (k, _) in all_zeros {
        let idx = ht[k];
        init_opened.insert(idx);

        opened[idx] = true;
        cnt_zeros += 1;
    }

    let mut all_pos: Vec<(usize, Vec<bool>)> = Vec::new();
    (
        dfs(
            0,
            cnt_zeros,
            0,
            &dist,
            &inv_ht,
            &flow,
            &mut opened,
            1,
            req_time,
            &mut all_pos,
        ),
        all_pos,
        init_opened,
    )
}

fn dfs(
    f: usize,
    nums: usize,
    valve: usize,
    dist: &Vec<Vec<usize>>,
    ht: &HashMap<&usize, &&str>,
    flow: &HashMap<&str, usize>,
    opened: &mut Vec<bool>,
    time: usize,
    end_time: usize,
    all_pos: &mut Vec<(usize, Vec<bool>)>,
) -> usize {
    all_pos.push((f, opened.clone()));

    if time >= 30 || nums == opened.len() {
        return f;
    }

    let mut max_flow = f;

    for diff_valve in 1..dist.len() {
        let dst = dist[valve][diff_valve];
        let this_time = time + dst + 1;

        if this_time >= end_time {
            continue;
        } else if !opened[diff_valve] {
            let this_f = flow[ht[&diff_valve]];

            opened[diff_valve] = true;
            max_flow = max(
                max_flow,
                dfs(
                    f + this_f * (end_time - this_time),
                    nums + 1,
                    diff_valve,
                    dist,
                    ht,
                    flow,
                    opened,
                    this_time,
                    end_time,
                    all_pos,
                ),
            );
            opened[diff_valve] = false;
        }
    }
    max_flow
}

fn get_dist<'a>(
    adj_list: &'a HashMap<&str, Vec<&str>>,
) -> (HashMap<&'a str, usize>, Vec<Vec<usize>>) {
    let mut ht: HashMap<&str, usize> = HashMap::new();

    let mut vertices: Vec<&&str> = adj_list.keys().collect();
    vertices.sort();

    let mut cnt = 0;

    for u in vertices {
        ht.insert(u, cnt);
        cnt += 1;
    }

    let mut dist = vec![vec![usize::MAX / 2; cnt]; cnt];

    for (u, all_v) in adj_list {
        for v in all_v {
            dist[ht[u]][ht[v]] = 1;
        }
        dist[ht[u]][ht[u]] = 0;
    }

    for k in 0..cnt {
        for i in 0..cnt {
            for j in 0..cnt {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }
    (ht, dist)
}

fn parse_input(lines: &Vec<String>) -> (HashMap<&str, usize>, HashMap<&str, Vec<&str>>) {
    let mut flow: HashMap<&str, usize> = HashMap::new();
    let mut path: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines {
        let init = split_by(line, " ");
        let valve = init[1];

        let flow_info = init[4].to_string();
        let init2 = split_by(&flow_info, "=");
        let f: usize = init2[1].strip_suffix(";").unwrap().parse().unwrap();

        let to_idx = init.iter().position(|&s| s == "to").unwrap();
        let mut connected: Vec<&str> = Vec::new();

        for i in to_idx + 2..init.len() {
            let v2 = init[i].strip_suffix(",").unwrap_or(init[i]);
            connected.push(v2);
        }

        flow.insert(valve, f);
        path.insert(valve, connected);
    }

    (flow, path)
}

fn split_by<'a>(s: &'a String, delim: &str) -> Vec<&'a str> {
    let split = s.split(delim);
    split.collect()
}
