pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        return MinStack { stack: Vec::new() };
    }

    pub fn push(&mut self, val: i32) {
        match self.stack.last() {
            Some(i) => {
                if i.1 < val {
                    self.stack.push((val, i.1));
                } else {
                    self.stack.push((val, val));
                }
            }
            None => {
                self.stack.push((val, val));
            }
        }
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    pub fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_stack() {
        let stack: MinStack = MinStack::new();
        assert![stack.stack.is_empty()];
    }

    #[test]
    fn test_push_to_stack() {
        let mut stack: MinStack = MinStack::new();
        stack.push(1);
        assert![!stack.stack.is_empty()];
    }

    #[test]
    fn test_pop_from_stack() {
        let mut stack: MinStack = MinStack::new();
        stack.push(1);
        stack.pop();
        assert![stack.stack.is_empty()];
    }

    #[test]
    fn test_top() {
        let mut stack: MinStack = MinStack::new();
        stack.push(1);
        assert![stack.top() == 1];
        stack.push(2);
        assert![stack.top() == 2];
    }

    #[test]
    fn test_get_min() {
        let mut stack: MinStack = MinStack::new();
        stack.push(1);
        stack.push(2);
        assert![stack.get_min() == 1];
    }

    #[test]
    fn test_get_min_2() {
        let mut stack: MinStack = MinStack::new();
        stack.push(7);
        stack.push(3);
        assert![stack.get_min() == 3];
        stack.push(-1);
        assert![stack.get_min() == -1];
    }

    #[test]
    fn test_get_min_after_popping_previous_min_val() {
        let mut stack: MinStack = MinStack::new();
        stack.push(7);
        stack.push(3);
        assert![stack.get_min() == 3];
        stack.push(-1);
        assert![stack.get_min() == -1];
        stack.pop();
        assert![stack.get_min() == 3];
    }
}
