use std::collections::{HashMap, BinaryHeap};


impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        
        let mut c = Self::counter(nums);
        let mut heap: BinaryHeap<(i32,i32)> = BinaryHeap::new();

        for (v,k) in c {
            heap.push((k,v));
        }

        let mut res: Vec<i32> = Vec::new();

        for _ in 0..k{

            if let Some((_,i)) = heap.pop() {
                res.push(i);
            }
            
        }

        res
    }

    pub fn counter(nums: Vec<i32>) -> HashMap<i32, i32> {
        let mut map: HashMap<i32,i32> = HashMap::new();

        for num in nums {
            *map.entry(num).or_insert(0) += 1
        }
        map
    }
}


