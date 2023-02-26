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

#[cfg(test)]
mod tests {
    use crate::solutions::queues::Solution;
    #[test]
    fn test_count_students() {
        let actual = Solution::count_students(vec![1, 0, 0, 1], vec![0, 1, 0, 1]);
        assert_eq!(actual, 0);
    }
}
