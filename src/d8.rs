use std::cmp::max;

pub fn p1(lines: &Vec<String>) -> usize {
    let m = lines.len();
    let n = lines.get(0).unwrap().len();
    let mut vis = 2 * m + 2 * n - 4;

    // dp[i][j] = (max from top, max from bottom, max from left, max from right)
    let mut dp = vec![vec![[0, 0, 0, 0]; m]; n];
    let mut trees = vec![vec![0; m]; n];

    for (i, line) in lines.iter().enumerate() {
        for (j, ch_sz) in line.chars().enumerate() {
            let size = ch_sz.to_digit(10).unwrap();
            trees[i][j] = size;
        }
    }

    // Max from top to bottom
    for j in 0..n {
        for i in 0..m {
            if i == 0 {
                dp[i][j][0] = trees[i][j];
            } else {
                dp[i][j][0] = max(dp[i - 1][j][0], trees[i - 1][j]);
            }
        }
    }

    // Max from bottom to top
    for j in 0..n {
        for i in (0..m).rev() {
            if i == m - 1 {
                dp[i][j][1] = trees[i][j];
            } else {
                dp[i][j][1] = max(dp[i + 1][j][1], trees[i + 1][j]);
            }
        }
    }

    // Max from left to right
    for i in 0..m {
        for j in 0..n {
            if j == 0 {
                dp[i][j][2] = trees[i][j];
            } else {
                dp[i][j][2] = max(dp[i][j - 1][2], trees[i][j - 1]);
            }
        }
    }

    // Max from right to left
    for i in 0..m {
        for j in (0..n).rev() {
            if j == n - 1 {
                dp[i][j][3] = trees[i][j];
            } else {
                dp[i][j][3] = max(dp[i][j + 1][3], trees[i][j + 1]);
            }
        }
    }

    for i in 1..m - 1 {
        for j in 1..n - 1 {
            if dp[i][j].iter().any(|e| trees[i][j] > *e) {
                vis += 1;
            }
        }
    }

    vis
}

pub fn p2(lines: &Vec<String>) -> usize {
    let m = lines.len();
    let n = lines.get(0).unwrap().len();
    let mut max_scene = 0;

    let mut trees = vec![vec![0; m]; n];

    for (i, line) in lines.iter().enumerate() {
        for (j, ch_sz) in line.chars().enumerate() {
            let size = ch_sz.to_digit(10).unwrap();
            trees[i][j] = size;
        }
    }

    for i in 1..m - 1 {
        for j in 1..n - 1 {
            let mut cur_scene = 1;

            for (di, dj) in [(1, 0), (-1, 0), (0, -1), (0, 1)] {
                let mut ddi = (i as i32) + di;
                let mut ddj = (j as i32) + dj;
                let mut dst = 1;
                while ddi > 0
                    && ddi < (m as i32 - 1)
                    && ddj > 0
                    && ddj < (n as i32 - 1)
                    && trees[i][j] > trees[ddi as usize][ddj as usize]
                {
                    ddi += di;
                    ddj += dj;
                    dst += 1;
                }
                cur_scene *= dst;
            }

            max_scene = max(max_scene, cur_scene);
        }
    }

    max_scene
}
