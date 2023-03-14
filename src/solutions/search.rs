use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        fn _search(nums: &[i32], left: usize, right: usize, target: i32) -> isize {
            if left > right {
                return -1;
            }
            let middle = (left + right) / 2;
            match target.cmp(&nums[middle]) {
                Ordering::Less => _search(nums, left, middle - 1, target),
                Ordering::Equal => return middle as isize,
                Ordering::Greater => _search(nums, middle + 1, right, target),
            }
        }

        let length = nums.len() - 1;
        let num = _search(&nums, 0, length, target);
        return num as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_binary_search() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let actual = Solution::binary_search(nums, 9);
        assert_eq!(actual, 4);
    }
}
