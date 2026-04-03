use std::cmp::max;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let ROW: usize = text1.len();
        let COL: usize = text2.len();

        let mut dp = vec![vec![0; COL + 1]; ROW + 1];

        let t1: Vec<char> = text1.chars().collect();
        let t2: Vec<char> = text2.chars().collect();

        for i in 1..ROW + 1 {
            for j in 1..COL + 1 {
                if t1[i - 1] == t2[j - 1] {
                    dp[i][j] = 1 + dp[i - 1][j - 1];
                } else {
                    dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }

        dp[ROW][COL]
    }

