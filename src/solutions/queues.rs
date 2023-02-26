struct Solution {}

impl Solution {
    pub fn count_students(mut students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
        let mut same_loop = 0;
        while sandwiches.len() > 0 && sandwiches.len() > same_loop {
            let student = students.remove(0);
            let sandwich = sandwiches.remove(0);
            if student != sandwich {
                same_loop += 1;
                sandwiches.insert(0, sandwich);
                students.push(student);
            } else {
                same_loop = 0;
            }
            println!("sameloop: {}", same_loop);
        }
        students.len() as i32
    }
}

struct MyStack {
    original: Vec<i32>,
    reversed: Vec<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self {
            original: vec![],
            reversed: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.original.insert(0, x);
        self.reversed.insert(self.reversed.len(), x);
    }

    fn pop(&mut self) -> i32 {
        self.reversed.remove(self.reversed.len() - 1);
        self.original.remove(0)
    }

    fn top(&self) -> i32 {
        self.original[0]
    }

    fn empty(&self) -> bool {
        self.reversed.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::queues::Solution;

    use super::MyStack;
    #[test]
    fn test_count_students() {
        let actual = Solution::count_students(vec![1, 0, 0, 1], vec![0, 1, 0, 1]);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_my_stack() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.empty(), false);
    }
}
