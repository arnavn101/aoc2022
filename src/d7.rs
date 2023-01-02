use std::collections::HashMap;

pub fn p1(lines: &Vec<String>) -> u32 {
    let ht: HashMap<String, u32> = get_fs(lines);
    let mut tot_sz = 0;

    for (_, size) in &ht {
        if *size <= 100000 {
            tot_sz += size;
        }
    }

    tot_sz
}

pub fn p2(lines: &Vec<String>) -> u32 {
    let ht: HashMap<String, u32> = get_fs(lines);
    let mut best_dir = 70000000;
    let needed_space = 30000000 - (best_dir - ht["/"]);

    for (_, size) in &ht {
        if *size >= needed_space && *size < best_dir {
            best_dir = *size;
        }
    }

    best_dir
}

pub fn get_fs(lines: &Vec<String>) -> HashMap<String, u32> {
    let mut pth: Vec<String> = Vec::new();
    let mut ht: HashMap<String, u32> = HashMap::from([(String::from("/"), 0)]);

    for line in lines {
        if line.starts_with("$") {
            let split = line.split(" ");
            let args: Vec<&str> = split.collect();

            match args[1] {
                "cd" => match args[2] {
                    "/" => {
                        pth.clear();
                    }
                    ".." => {
                        pth.pop();
                    }
                    dir => pth.push(dir.to_string()),
                },
                "ls" => (),
                _ => panic!("Unknown command"),
            }
        } else {
            let split = line.split(" ");
            let items: Vec<&str> = split.collect();

            if items[0] != "dir" {
                let sz: u32 = items[0].parse().unwrap();
                let mut acc = String::from("/");

                *ht.get_mut(&acc).unwrap() += sz;

                for dir in pth.iter() {
                    acc += dir;
                    if ht.contains_key(&acc) {
                        *ht.get_mut(&acc).unwrap() += sz;
                    } else {
                        ht.insert(acc.clone(), sz);
                    }
                    acc.push('/');
                }
            }
        }
    }

    ht
}
