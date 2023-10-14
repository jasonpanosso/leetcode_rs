use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

struct SmallestInfiniteSet {
    data: BinaryHeap<Reverse<i32>>,
    set: HashSet<i32>,
    smallest: i32,
}

impl SmallestInfiniteSet {
    pub fn new() -> Self {
        SmallestInfiniteSet {
            data: BinaryHeap::new(),
            set: HashSet::new(),
            smallest: 1,
        }
    }

    pub fn pop_smallest(&mut self) -> i32 {
        if self.data.is_empty() {
            self.smallest += 1;
            self.smallest - 1
        } else {
            let smallest = self.data.pop().unwrap().0;
            self.set.remove(&smallest);
            smallest
        }
    }

    pub fn add_back(&mut self, num: i32) {
        if self.smallest > num && !self.set.contains(&num) {
            self.set.insert(num);
            self.data.push(Reverse(num))
        }
    }
}

pub fn temp_please_compiler() {
    let mut smallest_infinite_set = SmallestInfiniteSet::new();
    smallest_infinite_set.add_back(1);
    smallest_infinite_set.pop_smallest();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut smallest_infinite_set = SmallestInfiniteSet::new();

        let mut output: Vec<i32> = vec![];
        smallest_infinite_set.add_back(2);
        output.push(smallest_infinite_set.pop_smallest());
        output.push(smallest_infinite_set.pop_smallest());
        output.push(smallest_infinite_set.pop_smallest());

        smallest_infinite_set.add_back(1);
        output.push(smallest_infinite_set.pop_smallest());
        output.push(smallest_infinite_set.pop_smallest());
        output.push(smallest_infinite_set.pop_smallest());

        let expected = vec![1, 2, 3, 1, 4, 5];
        assert_eq!(output, expected);
    }
}
