use kdam::tqdm;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub fn p1(lines: &Vec<String>) -> usize {
    let ht = parse_input(lines);
    let row = 2000000;
    find_covered(&ht, row).len()
}

pub fn p2(lines: &Vec<String>) -> usize {
    let ht = parse_input(lines);
    let c: i32 = 4000000;

    for y in tqdm!(0..c + 1) {
        let f = find_range(&ht, y, c as usize);

        if f != -1 {
            return (y as i64 + f * 4000000) as usize;
        }
    }

    0
}

fn find_covered(ht: &HashMap<(i32, i32), (i32, i32)>, check_row: i32) -> HashSet<i32> {
    let mut cov: HashSet<i32> = HashSet::new();

    for (sensor, beacon) in tqdm!(ht.iter()) {
        let dst = man_dist(sensor, beacon);
        let (x, y) = *sensor;
        let y_dst = (y - check_row).abs();

        for x_dst in 0..dst - y_dst + 1 {
            for e in [x + x_dst, x - x_dst] {
                cov.insert(e);
            }
        }

        if beacon.1 == check_row && cov.contains(&beacon.0) {
            cov.remove(&beacon.0);
        }
    }

    cov
}

fn find_range(ht: &HashMap<(i32, i32), (i32, i32)>, check_row: i32, c: usize) -> i64 {
    let mut filled: Vec<(i32, i32)> = Vec::new();

    for (sensor, beacon) in ht {
        let dst = man_dist(sensor, beacon);
        let (x, y) = *sensor;
        let y_dst = (y - check_row).abs();

        let x_dst = dst - y_dst;

        if x_dst <= 0 {
            continue;
        }

        let min_x = max(0, min(x + x_dst, x - x_dst));
        let max_x = min(c as i32, max(x + x_dst, x - x_dst));

        filled.push((min_x, max_x));
    }

    contains_point(&filled, 0, c as i32) as i64
}

fn contains_point(ranges: &Vec<(i32, i32)>, min_x: i32, max_x: i32) -> i32 {
    if ranges.iter().any(|(l, u)| min_x >= *l && max_x <= *u) {
        return -1;
    } else if min_x == max_x {
        return min_x;
    } else {
        let mid = (min_x + max_x) / 2;
        let quadrants = [(min_x, mid), (mid + 1, max_x)];
        for (ql, qh) in quadrants {
            let c = contains_point(ranges, ql, qh);
            if c != -1 {
                return c;
            }
        }
    }

    return -1;
}

fn man_dist(t1: &(i32, i32), t2: &(i32, i32)) -> i32 {
    (t1.0 - t2.0).abs() + (t1.1 - t2.1).abs()
}

fn parse_input(lines: &Vec<String>) -> HashMap<(i32, i32), (i32, i32)> {
    let mut ht: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    for line in lines {
        let split = line.split(":").map(|s| parse_coords(s));
        let items: Vec<(i32, i32)> = split.collect();
        ht.insert(items[0], items[1]);
    }

    ht
}

fn parse_coords(s: &str) -> (i32, i32) {
    let split = s.split(",");
    let c_split = split.map(|e| {
        let split2 = e.split("=");
        let c_split2: Vec<&str> = split2.collect();
        c_split2[1]
    });
    let coords: Vec<i32> = c_split.map(|e| e.parse().unwrap()).collect();
    (coords[0], coords[1])
}
