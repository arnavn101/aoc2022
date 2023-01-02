pub fn p1(lines: &Vec<String>) -> i32 {
    let mut num_pairs = 0;

    for line in lines {
        let split = line.split(",");
        let pairs: Vec<&str> = split.collect();

        let mut ppairs = [[-1, -1], [-1, -1]];

        for (i, p) in pairs.iter().enumerate() {
            let second_split = p.split("-");
            let nums: Vec<&str> = second_split.collect();

            for (j, s) in nums.iter().enumerate() {
                ppairs[i][j] = s.parse().unwrap();
            }
        }

        for k in 0..2 {
            if ppairs[k][0] <= ppairs[(k + 1) % 2][0] && ppairs[k][1] >= ppairs[(k + 1) % 2][1] {
                num_pairs += 1;
                break;
            }
        }
    }

    num_pairs
}

pub fn p2(lines: &Vec<String>) -> i32 {
    let mut num_pairs = 0;

    for line in lines {
        let split = line.split(",");
        let pairs: Vec<&str> = split.collect();

        let mut ppairs = [[-1, -1], [-1, -1]];

        for (i, p) in pairs.iter().enumerate() {
            let second_split = p.split("-");
            let nums: Vec<&str> = second_split.collect();

            for (j, s) in nums.iter().enumerate() {
                ppairs[i][j] = s.parse().unwrap();
            }
        }

        for k in 0..2 {
            let mut break_out = false;
            for l in 0..2 {
                if ppairs[k][l] >= ppairs[(k + 1) % 2][0] && ppairs[(k + 1) % 2][1] >= ppairs[k][l]
                {
                    num_pairs += 1;
                    break_out = true;
                    break;
                }
            }
            if break_out {
                break;
            }
        }
    }

    num_pairs
}
