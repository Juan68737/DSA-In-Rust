
use std::cmp::min;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let col = word2.len() + 1;
        let row = word1.len() + 1;
        let mut dp = vec![vec![0; col]; row];

        for i in 0..col {
            dp[0][i] = i as i32;
        }

        for i in 0..row {
            dp[i][0] = i as i32;
        }

        let w1: Vec<char> = word1.chars().collect();
        let w2: Vec<char> = word2.chars().collect();

        for r in 1..row {
            for c in 1..col {
                if w1[r - 1] == w2[c - 1] {
                    dp[r][c] = dp[r - 1][c - 1];
                } else {
                    dp[r][c] = 1 + min(dp[r - 1][c - 1], min(dp[r - 1][c], dp[r][c - 1]));
                }
            }
        }

        dp[row - 1][col - 1]
    }
}

