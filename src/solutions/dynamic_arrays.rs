struct Solution {}

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ans = nums.clone();
        ans.append(&mut nums);
        ans
    }
}
