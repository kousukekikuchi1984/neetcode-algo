impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        /* x: integer
         * +: sum previous two scores and append it
         * D: double the previous score
         * C: delete the last scores
         * */
        let mut queue: Vec<i32> = vec![];
        for operation in operations {
            let queue_length = queue.len();
            match operation.as_str() {
                "+" => {
                    let sum = queue[queue_length - 1] + queue[queue_length - 2];
                    queue.push(sum);
                }
                "D" => {
                    let multiply = queue[queue_length - 1] * 2;
                    queue.push(multiply)
                }
                "C" => {
                    queue.pop();
                }
                _ => queue.push(operation.parse::<i32>().unwrap()),
            }
        }
        queue.iter().sum()
    }

    pub fn is_valid(s: String) -> bool {
        let mut queue: Vec<char> = vec![];
        let map = HashMap::from([('(', ')'), ('{', '}'), ('[', ']')]);
        for char in s.chars() {
            if char == '(' || char == '{' || char == '[' {
                queue.push(char);
            } else {
                let prev = queue.pop();
                if prev == None {
                    return false;
                } else {
                    if char != map[&prev.unwrap()] {
                        return false;
                    }
                }
            }
        }
        queue.len() == 0
    }
}

// the code below is for min stack problem
struct MinStack {
    stack: Vec<i32>,
    min_values: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            stack: vec![],
            min_values: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        match self.min_values.get(self.min_values.len() - 1) {
            None => self.min_values.push(val),
            Some(min_value) => {
                if min_value > &val {
                    self.min_values.push(val);
                } else {
                    self.min_values.push(*min_value)
                }
            }
        }
    }

    fn pop(&mut self) {
        self.min_values.pop();
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    fn get_min(&self) -> i32 {
        self.min_values[self.min_values.len() - 1]
    }
}
