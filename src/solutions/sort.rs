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

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = nums.len() - k as usize;

        fn quick_select(nums: &mut [i32], start: usize, end: usize, k: usize) -> i32 {
            let pivot = nums[end];
            let mut cur = start;
            for i in start..end {
                if pivot > nums[i] {
                    nums.swap(cur, i);
                    cur += 1;
                }
            }
            nums.swap(cur, end);
            if cur > k as usize {
                return quick_select(nums, start, cur - 1, k);
            } else if cur < k {
                return quick_select(nums, cur + 1, end, k);
            } else {
                return nums[cur];
            }
        }
        let end = nums.len() - 1;
        return quick_select(&mut nums, 0, end, k);
    }

    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut colors = vec![0; 3];
        for num in nums.iter() {
            colors[*num as usize] += 1;
        }
        let mut cur = 0;
        for (color, &count) in colors.iter().enumerate() {
            for _ in 0..count {
                nums[cur] = color as i32;
                cur += 1;
            }
        }
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

    #[test]
    fn test_quick_select() {
        let mut lis: Vec<i32> = vec![3, 2, 1, 5, 6, 4];
        let actual = Solution::find_kth_largest(lis, 2);
        assert_eq!(actual, 5);

        let mut lis: Vec<i32> = vec![-1, 2, 0];
        let actual = Solution::find_kth_largest(lis, 3);
        assert_eq!(actual, -1);
    }

    #[test]
    fn test_sort_colors() {
        let mut lis: Vec<i32> = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut lis);
        assert_eq!(lis, vec![0, 0, 1, 1, 2, 2]);
    }
}
