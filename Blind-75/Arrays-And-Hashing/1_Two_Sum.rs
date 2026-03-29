use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut map: HashMap<i32,i32> = HashMap::new();

        for (i,num) in nums.iter().enumerate() {
            
            let check: i32 = target - *num;

            if let Some(&j) = map.get(&check) {
                return vec![j, i as i32]
            }

            map.insert(*num,i as i32);
        }

        vec![]
        
    }
}
