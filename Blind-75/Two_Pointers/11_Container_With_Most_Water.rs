use std::cmp::{max,min};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        
        let mut left: usize = 0;
        let mut right: usize = height.len() - 1;
        let mut res: i32 = 0;
        while left < right {
            res = max(res, (right-left) as i32 * min(height[left],height[right]));

            if height[left] < height[right] {
                left += 1;
            }
            else{
                right -=1
            }
        }
        res
    }
}


