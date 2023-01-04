use std::collections::HashSet;

pub fn p1(lines: &Vec<String>) -> usize {
    let mut h = [0, 0];
    let mut t = [0, 0];
    let mut vis: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    for line in lines {
        let split = line.split(" ");
        let items: Vec<&str> = split.collect();
        let mag: usize = items[1].parse().unwrap();

        for _ in 0..mag {
            match items[0] {
                "U" => h[1] += 1,
                "D" => h[1] -= 1,
                "L" => h[0] -= 1,
                "R" => h[0] += 1,
                _ => panic!("Unknown move"),
            }
            move_tail(&mut h, &mut t);
            vis.insert((t[0], t[1]));
        }
    }

    vis.len()
}

pub fn p2(lines: &Vec<String>) -> usize {
    let mut rope = [[0; 2]; 10];
    let mut vis: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    for line in lines {
        let split = line.split(" ");
        let items: Vec<&str> = split.collect();
        let mag: usize = items[1].parse().unwrap();

        for _ in 0..mag {
            match items[0] {
                "U" => rope[0][1] += 1,
                "D" => rope[0][1] -= 1,
                "L" => rope[0][0] -= 1,
                "R" => rope[0][0] += 1,
                _ => panic!("Unknown move"),
            }

            for j in 1..10 {
                let (start, end) = rope.split_at_mut(j);
                move_tail(&mut start[start.len() - 1], &mut end[0]);
            }

            vis.insert((rope[9][0], rope[9][1]));
        }
    }

    vis.len()
}

fn move_tail(h: &mut [i32; 2], t: &mut [i32; 2]) {
    let dist_x = h[0] - t[0];
    let dist_y = h[1] - t[1];

    if dist_x == 0 {
        if dist_y.abs() > 1 {
            if dist_y > 0 {
                t[1] += 1;
            } else {
                t[1] -= 1;
            }
        }
    } else if dist_y == 0 {
        if dist_x.abs() > 1 {
            if dist_x > 0 {
                t[0] += 1;
            } else {
                t[0] -= 1;
            }
        }
    } else if dist_x.abs() != 1 || dist_y.abs() != 1 {
        if dist_x > 0 && dist_y > 0 {
            t[0] += 1;
            t[1] += 1;
        } else if dist_x > 0 && dist_y < 0 {
            t[0] += 1;
            t[1] -= 1;
        } else if dist_x < 0 && dist_y > 0 {
            t[0] -= 1;
            t[1] += 1;
        } else {
            t[0] -= 1;
            t[1] -= 1;
        }
    }
}
