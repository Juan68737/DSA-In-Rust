use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {

        let mut set: HashSet<&i32> = HashSet::new();

        for num in &nums {
            let res: bool = set.insert(num);
            if res == false {
                return true;
            }
        }

        false
        
    }
}
