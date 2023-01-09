use std::cmp::{max, min};
use std::collections::HashSet;

pub fn p1(lines: &Vec<String>) -> usize {
    let mut obstacles = parse_input(lines);
    let mut units = 0;

    loop {
        let end = dfs((500, 0), &obstacles, false, -1);
        if end == (-1, -1) {
            break;
        }
        obstacles.insert(end);
        units += 1;
    }

    units
}

pub fn p2(lines: &Vec<String>) -> usize {
    let mut obstacles = parse_input(lines);
    let mut units = 0;
    let (_, max_y) = obstacles
        .iter()
        .max_by_key(|coord| coord.1)
        .unwrap()
        .clone();

    loop {
        let end = dfs((500, 0), &obstacles, true, max_y + 2);
        if end == (500, 0) {
            break;
        }
        obstacles.insert(end);
        units += 1;
    }

    units + 1
}

fn parse_input(lines: &Vec<String>) -> HashSet<(i32, i32)> {
    let mut rock_paths: Vec<Vec<(i32, i32)>> = Vec::new();

    for line in lines {
        let split = line.split("->");
        let items: Vec<&str> = split.collect();
        let mut v: Vec<(i32, i32)> = Vec::new();

        for item in items {
            let second_split = item.trim().split(",");
            let coords: Vec<&str> = second_split.collect();
            v.push((coords[0].parse().unwrap(), coords[1].parse().unwrap()));
        }

        rock_paths.push(v);
    }

    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();

    for r_path in rock_paths.iter() {
        for i in 0..r_path.len() {
            let (x1, y1) = r_path[i];
            if i == 0 {
                obstacles.insert((x1, y1));
            } else {
                let (x2, y2) = r_path[i - 1];
                for r in min(x1, x2)..max(x1, x2) + 1 {
                    for c in min(y1, y2)..max(y1, y2) + 1 {
                        obstacles.insert((r, c));
                    }
                }
            }
        }
    }

    obstacles
}

fn dfs(start: (i32, i32), obs: &HashSet<(i32, i32)>, is_p2: bool, floor_y: i32) -> (i32, i32) {
    let (x, y) = start;
    let dirs = [(0, 1), (-1, 1), (1, 1)];

    if !is_p2 {
        let mut filtered = obs.iter().filter(|(x2, y2)| *x2 == x && *y2 >= y);
        let e = filtered.next().unwrap_or_else(|| &(-1, -1));
        if *e == (-1, -1) {
            return e.clone();
        }
    }

    for (dx, dy) in dirs {
        let ddx = x + dx;
        let ddy = y + dy;

        if !obs.contains(&(ddx, ddy)) {
            if !is_p2 || ddy < floor_y {
                return dfs((ddx, ddy), obs, is_p2, floor_y);
            }
        }
    }

    start
}
