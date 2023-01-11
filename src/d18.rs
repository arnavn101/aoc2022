use std::collections::{HashSet, VecDeque};
use std::iter::zip;

pub fn p1(lines: &Vec<String>) -> u32 {
    let cubes = parse_input(lines);
    surface_area(&cubes)
}

pub fn p2(lines: &Vec<String>) -> u32 {
    let cubes = parse_input(lines);
    let (min_coord, max_coord) = get_min_max(&cubes);
    let mut sides = 0;

    let mut q = VecDeque::from([(min_coord[0], min_coord[1], min_coord[2])]);
    let mut v = cubes.clone();

    while q.len() > 0 {
        let c = q.pop_front().unwrap();
        for cube in get_neighbors(&c) {
            if zip(min_coord, zip(max_coord, [cube.0, cube.1, cube.2]))
                .any(|(mn, (mx, coord))| mn > coord || mx < coord)
            {
                continue;
            }

            if cubes.contains(&cube) {
                sides += 1;
            }

            if !v.contains(&cube) {
                v.insert(cube);
                q.push_back(cube);
            }
        }
    }

    sides
}

fn surface_area(cubes: &HashSet<(i32, i32, i32)>) -> u32 {
    let mut sides = 0;

    for c in cubes.iter() {
        let neighbors = get_neighbors(c);

        let mut cur_sides = 6;
        for p in neighbors {
            if cubes.contains(&p) {
                cur_sides -= 1;
            }
        }
        sides += cur_sides;
    }

    sides
}

fn get_min_max(cubes: &HashSet<(i32, i32, i32)>) -> ([i32; 3], [i32; 3]) {
    let min_x = cubes.iter().map(|c| c.0).min().unwrap() - 1;
    let min_y = cubes.iter().map(|c| c.1).min().unwrap() - 1;
    let min_z = cubes.iter().map(|c| c.2).min().unwrap() - 1;

    let max_x = cubes.iter().map(|c| c.0).max().unwrap() + 1;
    let max_y = cubes.iter().map(|c| c.1).max().unwrap() + 1;
    let max_z = cubes.iter().map(|c| c.2).max().unwrap() + 1;

    ([min_x, min_y, min_z], [max_x, max_y, max_z])
}

fn get_neighbors((x, y, z): &(i32, i32, i32)) -> [(i32, i32, i32); 6] {
    [
        (x + 1, *y, *z),
        (x - 1, *y, *z),
        (*x, y + 1, *z),
        (*x, y - 1, *z),
        (*x, *y, z + 1),
        (*x, *y, z - 1),
    ]
}

fn parse_input(lines: &Vec<String>) -> HashSet<(i32, i32, i32)> {
    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();

    for line in lines {
        let split = line.split(",");
        let v: Vec<i32> = split.map(|s| s.parse().unwrap()).collect();
        cubes.insert((v[0], v[1], v[2]));
    }

    cubes
}
