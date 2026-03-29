use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        
        let mut c: HashMap<Vec<char>, Vec<String>> = HashMap::new();

        for s in strs {
            let mut value: Vec<char> = s.chars().collect();
            value.sort();

            c.entry(value).or_default().push(s)
        }

        let mut res: Vec<Vec<String>> = Vec::new();

        for (k,v) in c {
            res.push(v);
        }

        res
    }
}


        
