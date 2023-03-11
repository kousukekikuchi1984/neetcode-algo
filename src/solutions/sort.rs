struct Solution {}

impl Solution {
    pub fn insert_sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.to_vec();
        for i in 1..nums.len() {
            let mut j = i;
            while j > 0 && nums[j - 1] > nums[j] {
                let current = nums[j];
                nums[j] = nums[j - 1];
                nums[j - 1] = current;
                j -= 1;
            }
        }
        return nums;
    }

    pub fn merge_sort_array(nums: &mut Vec<i32>) -> Vec<i32> {
        fn merge_sort(nums: &mut Vec<i32>, start: usize, end: usize) -> Vec<i32> {
            println!("length: {}, start: {}, end: {}", nums.len(), start, end);
            if end - start + 1 <= 1 {
                return nums[start..=end].to_vec();
            }
            let middle = (start + end) / 2;
            let left = merge_sort(nums, start, middle);
            let right = merge_sort(nums, middle + 1, end);
            merge(left, right)
        }
        fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
            let mut result = vec![0; left.len() + right.len()];
            let mut curleft = 0;
            let mut curright = 0;
            let mut curresult = 0;
            while curleft < left.len() && curright < right.len() {
                if left[curleft] > right[curright] {
                    result[curresult] = right[curright];
                    curright += 1;
                } else {
                    result[curresult] = left[curleft];
                    curleft += 1;
                }
                curresult += 1;
            }
            // 残ったものの処理
            while curleft < left.len() {
                result[curresult] = left[curleft];
                curresult += 1;
                curleft += 1;
            }
            while curright < right.len() {
                result[curresult] = right[curright];
                curresult += 1;
                curright += 1;
            }

            return result;
        }
        merge_sort(nums, 0, nums.len() - 1)
    }

    pub fn quick_sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        fn quick_sort(nums: &mut [i32], start: usize, end: usize) {
            if (end + 1 - start) <= 1 {
                return;
            }
            let pivot_idx = partition(nums, start, end);
            if pivot_idx > 0 {
                quick_sort(nums, start, pivot_idx - 1);
            }
            quick_sort(nums, pivot_idx + 1, end);
        }

        fn partition(nums: &mut [i32], start: usize, end: usize) -> usize {
            let pivot = nums[end];
            let mut cur = start;
            for i in start..end {
                if pivot > nums[i] {
                    nums.swap(cur, i);
                    cur += 1;
                }
            }
            nums.swap(cur, end);
            return cur;
        }

        let end = nums.len() - 1;
        quick_sort(&mut nums, 0, end);
        return nums;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_insert_sort_array() {
        let lis = vec![5, 4, 3, 2, 1];
        let actual = Solution::insert_sort_array(lis);
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_merge_sort_array() {
        let mut lis: Vec<i32> = vec![5, 4, 3, 2, 1];
        let actual = Solution::merge_sort_array(&mut lis);
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_quick_sort_array() {
        let mut lis: Vec<i32> = vec![5, 4, 3, 2, 1];
        let actual = Solution::quick_sort_array(lis);
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(actual, expected);
    }
}
