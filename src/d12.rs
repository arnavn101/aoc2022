use std::cmp::min;
use std::collections::{HashSet, VecDeque};

pub fn p1(lines: &Vec<String>) -> usize {
    let m = lines.len();
    let n = lines[0].len();
    let mut mat = vec![vec!['.'; n]; m];
    let mut start = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            mat[i][j] = ch;

            if ch == 'S' {
                start = (i, j)
            }
        }
    }

    bfs(&mat, start.0, start.1)
}

pub fn p2(lines: &Vec<String>) -> usize {
    let m = lines.len();
    let n = lines[0].len();
    let mut mat = vec![vec!['.'; n]; m];
    let mut all_starts: Vec<(usize, usize)> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            mat[i][j] = ch;

            if ch == 'S' || ch == 'a' {
                all_starts.push((i, j));
            }
        }
    }

    let mut min_steps = usize::MAX;

    for (x, y) in all_starts {
        min_steps = min(min_steps, bfs(&mat, x, y));
    }

    min_steps
}

fn bfs(mat: &Vec<Vec<char>>, sx: usize, sy: usize) -> usize {
    let m = mat.len() as i32;
    let n = mat[0].len() as i32;
    const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];
    let mut q = VecDeque::from([(0, sx, sy)]);
    let mut v = HashSet::from([(sx, sy)]);

    while !q.is_empty() {
        let (s, x, y) = q.pop_front().unwrap();
        let mut ch = mat[x][y];

        if ch == 'E' {
            return s;
        }

        for (dx, dy) in DIRS {
            let ddx = (x as i32) + dx;
            let ddy = (y as i32) + dy;

            if ddx < 0 || ddy < 0 || ddx == m || ddy == n {
                continue;
            }

            let uddx = ddx as usize;
            let uddy = ddy as usize;

            let tup = (uddx, uddy);
            let ch2 = match mat[uddx][uddy] {
                'E' => 'z',
                'S' => 'a',
                ch => ch,
            };

            ch = match ch {
                'S' => 'a',
                ch => ch,
            };

            if v.contains(&tup) || ch2 as i32 - ch as i32 > 1 {
                continue;
            }

            v.insert(tup);
            q.push_back((s + 1, uddx, uddy));
        }
    }

    usize::MAX
}
