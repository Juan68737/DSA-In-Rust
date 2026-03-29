use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        
        let mut c = Self::counter(&s);

        for i in t.chars() {
            *c.entry(i).or_insert(0) -= 1;
        }

        for (_,v) in c {
            if v != 0 {
                return false
            }
        }
        true
    }

    pub fn counter(s: &String) -> HashMap<char, i32> {
        let mut map: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        map
    }
}
