struct CQueue {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl CQueue {
    fn new() -> Self {
        CQueue {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.left.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if self.right.is_empty() {
            self.left.reverse();
            self.right = self.left.clone();
            self.left = Vec::new();
        }
        if let Some(v) = self.right.pop() {
            v
        } else {
            -1
        }
    }
}
