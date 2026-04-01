use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut first = 0;
        let mut res: HashSet<Vec<i32>> = HashSet::new();

        while first < nums.len().saturating_sub(2) {
            if first > 0 && nums[first] == nums[first - 1] {
                first += 1;
                continue;
            }

            let mut hashmap: HashSet<i32> = HashSet::new();

            for second in first + 1..nums.len() {
                let third = -nums[first] - nums[second];

                if hashmap.contains(&third) {
                    let mut triple: Vec<i32> = vec![nums[first], nums[second], third];
                    triple.sort();
                    res.insert(triple);
                }

                hashmap.insert(nums[second]);
            }

            first += 1;
        }

        let mut r: Vec<Vec<i32>> = Vec::new();
        for triple in res {
            r.push(triple);
        }

        r
    }
}
