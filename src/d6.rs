use std::collections::HashMap;

pub fn p1(lines: &Vec<String>) -> i32 {
    nth_marker(lines, 4)
}

pub fn p2(lines: &Vec<String>) -> i32 {
    nth_marker(lines, 14)
}

pub fn nth_marker(lines: &Vec<String>, n: usize) -> i32 {
    let mut cnt_chars: HashMap<char, u32> = HashMap::new();
    let mut all_chars = lines.get(0).unwrap().chars();

    for (i, ch) in all_chars.clone().enumerate() {
        if cnt_chars.contains_key(&ch) {
            *cnt_chars.get_mut(&ch).unwrap() += 1;
        } else {
            cnt_chars.insert(ch, 1);
        }

        if i >= n {
            let ch_rem = all_chars.next().unwrap();
            *cnt_chars.get_mut(&ch_rem).unwrap() -= 1;
            if cnt_chars[&ch_rem] == 0 {
                cnt_chars.remove(&ch_rem);
            }
        }

        if cnt_chars.len() == n {
            return (i + 1) as i32;
        }
    }

    -1
}
