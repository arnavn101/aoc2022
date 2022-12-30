use std::collections::HashSet;

pub fn p1(lines: &Vec<String>) -> i32 {
    let mut tot = 0;

    for line in lines {
        let mut h1: HashSet<char> = HashSet::new();
        let mut h2: HashSet<char> = HashSet::new();
        let cur_len = line.chars().count();

        for (i, ch) in line.chars().enumerate() {
            if i < cur_len / 2 {
                h1.insert(ch);
            } else {
                h2.insert(ch);
            }
        }

        let inter: HashSet<&char> = h1.intersection(&h2).collect();
        assert!(inter.len() == 1);

        for ch in inter {
            let num_value: Vec<char> = ch.to_lowercase().collect();
            tot += *num_value.get(0).unwrap() as i32 - 96;

            if ch.is_uppercase() {
                tot += 26;
            }
        }
    }

    tot
}

pub fn p2(lines: &Vec<String>) -> i32 {
    let mut tot = 0;
    let mut all_sets: [HashSet<char>; 3] = [HashSet::new(), HashSet::new(), HashSet::new()];

    for (i, line) in lines.iter().enumerate() {
        let j = i % 3;

        for ch in line.chars() {
            all_sets[j].insert(ch);
        }

        if j != 2 {
            continue;
        }

        let inter = all_sets
            .iter()
            .skip(1)
            .fold(all_sets[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
            });

        assert!(inter.len() == 1);

        for ch in inter {
            let num_value: Vec<char> = ch.to_lowercase().collect();
            tot += *num_value.get(0).unwrap() as i32 - 96;

            if ch.is_uppercase() {
                tot += 26;
            }
        }

        for k in 0..all_sets.len() {
            all_sets[k].clear();
        }
    }

    tot
}
