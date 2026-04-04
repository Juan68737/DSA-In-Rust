impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        use std::collections::HashMap;

        fn is_power_of_two(x: i32) -> bool {
            x > 0 && (x & (x - 1)) == 0
        }

        fn opt(r: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if r == 0 {
                return 0;
            }

            if is_power_of_two(r) {
                return 1;
            }

            if let Some(&ans) = memo.get(&r) {
                return ans;
            }

            let exp = 31 - r.leading_zeros() as i32;
            let low = 2_i32.pow(exp as u32);
            let high = 2_i32.pow((exp + 1) as u32);

            let ans = 1 + std::cmp::min(
                opt(r - low, memo),
                opt(high - r, memo),
            );

            memo.insert(r, ans);
            ans
        }

        let mut memo = HashMap::new();
        opt(n, &mut memo)
    }
}
