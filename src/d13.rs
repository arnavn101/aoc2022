use core::cmp::Ordering;
use eval::{eval, Value};
use std::collections::HashSet;

pub fn p1(lines: &Vec<String>) -> usize {
    let mut vec = Vec::new();

    for i in (0..lines.len()).step_by(3) {
        vec.push((parse_array(&lines[i]), parse_array(&lines[i + 1])));
    }

    let mut sum_indices = 0;

    for (pair_index, (arr1, arr2)) in vec.iter().enumerate() {
        let ans = check_arrays(&arr1, &arr2) as usize;
        if ans == 1 {
            sum_indices += pair_index + 1;
        }
    }

    sum_indices
}

pub fn p2(lines: &Vec<String>) -> usize {
    let mut vec = Vec::new();

    for i in (0..lines.len()).step_by(3) {
        vec.push(parse_array(&lines[i]));
        vec.push(parse_array(&lines[i + 1]));
    }

    vec.push(parse_array(&"[[2]]".to_string()));
    vec.push(parse_array(&"[[6]]".to_string()));

    vec.sort_by(|a, b| partial_cmp(b, a));

    let mut prod = 1;
    let div_packets = HashSet::from([2, 6]);

    for (i, arr) in vec.iter().enumerate() {
        if arr.is_array() {
            let r = arr.as_array().unwrap();

            if r.len() == 1 && r[0].is_array() {
                let s = r[0].as_array().unwrap();
                if s.len() == 1 && s[0].is_number() {
                    let t = s[0].as_i64().unwrap() as i32;
                    if div_packets.contains(&t) {
                        prod *= i + 1;
                    }
                }
            }
        }
    }

    prod
}

fn partial_cmp(v1: &Value, v2: &Value) -> Ordering {
    let res = check_arrays(v1, v2);
    match res {
        1 => Ordering::Greater,
        -1 => Ordering::Less,
        _ => Ordering::Equal,
    }
}

fn parse_array(line: &String) -> Value {
    let parsed = line.replace("[", "array(").replace("]", ")");
    eval(&parsed).unwrap()
}

fn check_arrays(arr1: &Value, arr2: &Value) -> i32 {
    if arr1.is_number() && arr2.is_number() {
        let e1 = arr1.as_u64().unwrap();
        let e2 = arr2.as_u64().unwrap();

        if e1 < e2 {
            return 1;
        } else if e1 == e2 {
            return 0;
        } else {
            return -1;
        }
    } else if arr1.is_array() && !arr2.is_array() {
        return check_arrays(arr1, &num_to_array(arr2));
    } else if arr2.is_array() && !arr1.is_array() {
        return check_arrays(&num_to_array(arr1), arr2);
    }

    let a1 = arr1.as_array().unwrap();
    let a2 = arr2.as_array().unwrap();

    for (e1, e2) in a1.iter().zip(a2.iter()) {
        let checked = check_arrays(e1, e2);
        if checked == 1 {
            return 1;
        } else if checked == -1 {
            return -1;
        }
    }

    if a1.len() < a2.len() {
        return 1;
    } else if a1.len() == a2.len() {
        return 0;
    } else {
        return -1;
    }
}

fn num_to_array(num: &Value) -> Value {
    let str_repr = num.as_u64().unwrap().to_string();
    let expr = "array(".to_owned() + str_repr.as_str() + ")";
    eval(expr.as_str()).unwrap()
}
