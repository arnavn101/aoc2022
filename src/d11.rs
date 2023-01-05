use std::collections::{HashMap, VecDeque};

pub fn p1(lines: &Vec<String>) -> u64 {
    get_level(lines, 20, 3)
}

pub fn p2(lines: &Vec<String>) -> u64 {
    get_level(lines, 10000, 1)
}

fn get_level(lines: &Vec<String>, rounds: usize, div: i32) -> u64 {
    let mut monkeys: Vec<(VecDeque<i32>, HashMap<&str, i32>)> = Vec::new();
    let mut ht: HashMap<&str, i32> = HashMap::new();
    let mut items: VecDeque<i32> = VecDeque::new();
    let mut super_modulo: u64 = 1;

    for line in lines {
        if line.len() == 0 {
            monkeys.push((items.clone(), ht.clone()));
            items.clear();
            ht.clear();
            continue;
        }

        let split = line.split(" ");
        let split_vec: Vec<&str> = split.filter(|e| e.len() > 0).collect();

        match split_vec[0] {
            "Starting" => {
                for i in 2..split_vec.len() {
                    let w = split_vec[i].strip_suffix(",").unwrap_or(split_vec[i]);
                    items.push_back(w.parse().unwrap());
                }
            }
            "Operation:" => {
                let op = match split_vec[4] {
                    "+" => 0,
                    "*" => 1,
                    _ => 2,
                };
                ht.insert("op", op);
                ht.insert("operand", split_vec[5].parse().unwrap_or(-1));
            }
            "Test:" => {
                ht.insert("div", split_vec[3].parse().unwrap());
                super_modulo *= ht["div"] as u64;
            }
            "If" => {
                ht.insert(
                    split_vec[1].strip_suffix(":").unwrap(),
                    split_vec[5].parse().unwrap(),
                );
            }
            _ => (),
        }
    }

    let mut inspected = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let (m_items, m_ht) = monkeys[i].clone();

            for item in m_items {
                let mut worry = perform_op(item as u64, m_ht["op"], m_ht["operand"]) / (div as u64);
                worry = worry % super_modulo;
                let mut receiver = m_ht["false"];
                if worry % (m_ht["div"] as u64) == 0 {
                    receiver = m_ht["true"];
                }
                monkeys[receiver as usize].0.push_back(worry as i32);
                inspected[i] += 1;
            }

            monkeys[i].0.clear();
        }
    }

    inspected.sort_by(|a, b| b.cmp(a));
    inspected[0] * inspected[1]
}

fn perform_op(num1: u64, op: i32, operand: i32) -> u64 {
    let num2 = match operand {
        -1 => num1,
        other => other as u64,
    };

    match op {
        0 => num1 + num2,
        1 => num1 * num2,
        _ => 0,
    }
}
