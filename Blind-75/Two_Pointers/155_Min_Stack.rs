use std::cmp::min;

struct MinStack {
    stack: Vec<[i32; 2]>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push([val, val]);
        } else {
            let m = min(val, self.stack.last().unwrap()[1]);
            self.stack.push([val, m]);
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap()[0]
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap()[1]
    }
}
