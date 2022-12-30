use std::collections::{HashMap, HashSet};

pub fn p1(lines: &Vec<String>) -> i32 {
    let mut total_score = 0;

    let player_mapping = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);
    let shape_score = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let winning_set = HashSet::from([("A", "Y"), ("B", "Z"), ("C", "X")]);

    for line in lines {
        let split = line.split(" ");
        let vec: Vec<&str> = split.collect();

        let mut cur_score = shape_score[vec[1]];
        let cur_tuple = (vec[0], vec[1]);

        if player_mapping[vec[0]].eq(vec[1]) {
            cur_score += 3;
        } else if winning_set.contains(&cur_tuple) {
            cur_score += 6;
        }

        total_score += cur_score;
    }

    total_score
}

pub fn p2(lines: &Vec<String>) -> i32 {
    let mut total_score = 0;
    let player_mapping = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);

    let shape_score = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let winning_score = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);

    let win_mapping = HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")]);
    let lose_mapping = HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")]);

    for line in lines {
        let split = line.split(" ");
        let vec: Vec<&str> = split.collect();

        match vec[1] {
            "X" => total_score += shape_score[lose_mapping[vec[0]]],
            "Y" => total_score += shape_score[player_mapping[vec[0]]],
            "Z" => total_score += shape_score[win_mapping[vec[0]]],
            _ => (),
        };

        total_score += winning_score[vec[1]];
    }

    total_score
}
