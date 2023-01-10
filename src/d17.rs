use kdam::format::divmod;
use std::cmp::max;
use std::collections::{HashMap, HashSet};

pub fn p1(lines: &Vec<String>) -> i64 {
    tower_length(lines, 2022)
}

pub fn p2(lines: &Vec<String>) -> i64 {
    tower_length(lines, 1000000000000)
}

fn tower_length(lines: &Vec<String>, last_rock: usize) -> i64 {
    let jet: Vec<char> = lines[0].chars().collect();

    let rocks: [Vec<(i32, i32)>; 5] = [
        Vec::from([(0, 0), (0, 1), (0, 2), (0, 3)]),
        Vec::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
        Vec::from([(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]),
        Vec::from([(0, 0), (1, 0), (2, 0), (3, 0)]),
        Vec::from([(0, 0), (0, 1), (1, 0), (1, 1)]),
    ];

    let jet_map = HashMap::from([('>', 1), ('<', -1)]);

    let mut existing_rocks: HashSet<(i32, i32)> = HashSet::from([
        (-1, 0),
        (-1, 1),
        (-1, 2),
        (-1, 3),
        (-1, 4),
        (-1, 5),
        (-1, 6),
    ]);

    let mut top_rock: i64 = -1;
    let mut jet_idx = 0;
    let mut cache: HashMap<(usize, usize), (usize, i64)> = HashMap::new();

    for cur_rock in 0..last_rock + 1 {
        let r = rocks[cur_rock % rocks.len()].clone();
        let mut start_rock: Vec<(i32, i32)> = r
            .iter()
            .map(|(i, j)| (i + top_rock as i32 + 4, j + 2))
            .collect();
        let mut is_jet = true;

        let key = (cur_rock % rocks.len(), jet_idx % jet.len());

        if cache.contains_key(&key) {
            let (prev_rock, prev_top) = cache[&key];
            let (d, m) = divmod(last_rock - cur_rock, cur_rock - prev_rock);
            if m == 0 {
                top_rock += (top_rock - prev_top as i64) * (d as i64);
                break;
            }
        } else {
            cache.insert(key, (cur_rock, top_rock));
        }

        while is_jet
            || !start_rock
                .iter()
                .any(|(i, j)| existing_rocks.contains(&(i - 1, *j)))
        {
            let mut shift = (-1, 0);

            if is_jet {
                shift = (0, jet_map[&jet[jet_idx % jet.len()]]);
                jet_idx += 1;
            }

            let next_rock = start_rock.iter().map(|(i, j)| (i + shift.0, j + shift.1));

            if next_rock
                .clone()
                .all(|(i, j)| j >= 0 && j < 7 && !existing_rocks.contains(&(i, j)))
            {
                start_rock = next_rock.collect();
            }

            is_jet = !is_jet;
        }

        for tup in start_rock.iter() {
            top_rock = max(top_rock, tup.0 as i64);
            assert!(!existing_rocks.contains(tup));
            existing_rocks.insert(*tup);
        }
    }

    top_rock + 1
}

// fn visual_rocks(list_rocks: &HashSet<(i32, i32)>) {
//     let max_rock = list_rocks.iter().max_by_key(|(i, _)| i).unwrap().0;

//     for i in (0..max_rock + 1).rev() {
//         for j in 0..7 {
//             if list_rocks.contains(&(i, j)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
// }
