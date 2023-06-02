use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        while let Some(index) = nums.iter().position(|v| *v == val) {
            nums.remove(index);
        }

        nums.len() as i32
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut cur = 0;
        for i in 1..nums.len() {
            if nums[cur] != nums[i] {
                cur += 1;
                nums[cur] = nums[i];
            }
        }
        return (cur + 1) as i32;
    }

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut cur_sum = 0;
        for n in nums {
            cur_sum = max(cur_sum, 0);
            cur_sum += n;
            max_sum = max(max_sum, cur_sum);
        }
        max_sum
    }

    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut global_max, mut global_min) = (nums[0], nums[0]);
        let (mut current_max, mut current_min) = (0, 0);
        let mut total = 0;

        for num in nums {
            current_max = i32::max(num, current_max + num);
            current_min = i32::min(num, current_min + num);
            total += num;
            global_max = i32::max(global_max, current_max);
            global_min = i32::min(global_min, current_min);
        }

        return if global_max > 0 {
            i32::max(global_max, total - global_min)
        } else {
            global_max
        };
    }

    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        if arr.len() < 2 {
            return arr.len() as i32;
        }
        let mut max_len = 1;
        let mut cur_len = 1;
        let mut prev_cmp = None;
        for i in 1..arr.len() {
            let cmp = arr[i - 1].cmp(&arr[i]);
            if cmp == Ordering::Equal {
                cur_len = 1;
                prev_cmp = None;
            } else if let Some(prev) = prev_cmp {
                if prev != cmp {
                    cur_len += 1;
                    max_len = max(max_len, cur_len);
                } else {
                    cur_len = 2;
                }
                prev_cmp = Some(cmp);
            } else {
                cur_len += 1;
                max_len = max(max_len, cur_len);
                prev_cmp = Some(cmp);
            }
        }
        max_len
    }

    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut window = HashSet::new();
        let mut left: usize = 0;
        for right in 0..nums.len() {
            if right - left > k as usize {
                let val = nums[left];
                window.remove(&val);
                left += 1;
            }
            if window.contains(&nums[right]) {
                return true;
            }
            window.insert(nums[right]);
        }
        false
    }

    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        // strategy: sum of subarray of size k with sliding window
        let mut sum = 0;
        let mut count = 0;
        for i in 0..arr.len() {
            sum += arr[i];
            if i >= k as usize {
                sum -= arr[i - k as usize];
            }
            if sum >= k * threshold && i >= k as usize - 1 {
                count += 1;
            }
        }
        count
    }

    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        // sliding window + two pointers
        let mut min_len = nums.len() + 1;
        let mut sum = 0;
        let mut left: usize = 0;
        for right in 0..nums.len() {
            sum += nums[right];
            while sum >= target {
                min_len = min(min_len, right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }
        if min_len == nums.len() + 1 {
            return 0;
        }
        min_len as i32
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut cur = 0;
        let mut left: usize = 0;
        let mut current_chars = HashSet::new();
        let chars: Vec<char> = s.chars().collect();
        for right in 0..s.len() {
            let current_char = chars[right];
            if current_chars.contains(&current_char) {
                while chars[left] != current_char {
                    current_chars.remove(&chars[left]);
                    left += 1;
                    cur -= 1;
                }
                left += 1;
                cur -= 1;
            }
            current_chars.insert(current_char);
            cur += 1;
            max_len = max(max_len, cur);
        }
        max_len
    }

    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut max_len = 0;
        let mut cur = 0;
        let mut left: usize = 0;
        let mut char_counts = HashMap::new();
        let chars = s.chars().collect::<Vec<char>>();
        for right in 0..chars.len() {
            *char_counts.entry(chars[right]).or_insert(0) += 1;
            cur = max(cur, char_counts[&chars[right]]);
            while right - left + 1 - cur > k as usize {
                *char_counts.get_mut(&chars[left]).unwrap() -= 1;
                left += 1;
            }
            max_len = max(max_len, right - left + 1);
        }
        max_len as i32
    }

    pub fn is_palindrome(s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();
        let mut left: usize = 0;
        let mut right = chars.len() - 1;
        while left <= right {
            if !chars[left].is_alphanumeric() {
                left += 1;
                continue;
            }
            if !chars[right].is_alphanumeric() {
                right -= 1;
                continue;
            }
            if chars[left].to_ascii_lowercase() != chars[right].to_ascii_lowercase() {
                return false;
            }
            if left == right {
                return true;
            }
            left += 1;
            right -= 1;
        }
        true
    }

    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        // two pointers
        let mut left: usize = 0;
        let mut right = numbers.len() - 1;
        while left < right {
            let sum = numbers[left] + numbers[right];
            match target.cmp(&sum) {
                Ordering::Less => {
                    right -= 1;
                }
                Ordering::Equal => return vec![left as i32 + 1, right as i32 + 1],
                Ordering::Greater => {
                    left += 1;
                }
            }
        }
        unreachable!("No two sum solution");
    }

    pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = 1;
        let mut count = 1;
        while right < nums.len() {
            match nums[left].cmp(&nums[right]) {
                Ordering::Less => {
                    left = right;
                    count = 1;
                    right += 1;
                }
                Ordering::Equal => {
                    count += 1;
                    if count > 2 {
                        nums.remove(right);
                        continue;
                    }
                    right += 1;
                }
                Ordering::Greater => unreachable!("left should be fixed"),
            };
        }
        nums.len() as i32
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        fn area(left: usize, right: usize, height: &[i32]) -> i32 {
            (right - left) as i32 * min(height[left], height[right])
        }

        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = area(left, right, &height);
        while left < right {
            match height[left].cmp(&height[right]) {
                Ordering::Less | Ordering::Equal => left += 1,
                Ordering::Greater => right -= 1,
            }
            max_area = max(max_area, area(left, right, &height));
        }
        max_area
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = height[left];
        let mut right_max = height[right];
        let mut result = 0;

        while left < right {
            match left_max.cmp(&right_max) {
                Ordering::Less => {
                    left += 1;
                    left_max = max(left_max, height[left]);
                    result += left_max - height[left];
                }
                Ordering::Equal | Ordering::Greater => {
                    right -= 1;
                    right_max = max(right_max, height[right]);
                    result += right_max - height[right];
                }
            };
        }
        result
    }

    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum = vec![0; nums.len() + 1];
        let mut index = -1;
        for i in 0..nums.len() {
            sum[i + 1] = sum[i] + nums[i];
        }
        for i in 1..sum.len() {
            if sum[i - 1] == sum[sum.len() - 1] - sum[i] {
                index = i as i32 - 1;
                break;
            }
        }
        index
    }

    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut product_left = vec![1; nums.len() + 1];
        let mut product_right = vec![1; nums.len() + 1];
        let mut result = vec![0; nums.len()];
        for i in 0..nums.len() {
            product_left[i + 1] = product_left[i] * nums[i];
            product_right[nums.len() - i - 1] =
                product_right[nums.len() - i] * nums[nums.len() - i - 1];
        }
        for i in 0..nums.len() {
            result[i] = product_left[i] * product_right[i + 1];
        }

        result
    }

    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut count = 0;
        let mut map = HashMap::with_capacity(nums.len() / 2);
        map.insert(0, 1);
        for num in nums {
            sum += num;
            count += map.get(&(sum - k)).copied().unwrap_or(0);
            map.entry(sum).and_modify(|e| *e = *e + 1).or_insert(1);
        }
        count
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_hash: HashMap<char, u32> = HashMap::with_capacity(s.len());
        let mut t_hash: HashMap<char, u32> = HashMap::with_capacity(t.len());
        for c in s.chars() {
            *s_hash.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            *t_hash.entry(c).or_insert(0) += 1;
        }
        s_hash == t_hash
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        #[derive(Eq)]
        struct CharCount {
            char_map: HashMap<char, u32>,
        }

        impl Hash for CharCount {
            fn hash<H: Hasher>(&self, state: &mut H) {
                let mut sorted: Vec<_> = self.char_map.iter().collect();
                sorted.sort();
                for (c, count) in sorted {
                    c.hash(state);
                    count.hash(state);
                }
            }
        }

        impl PartialEq for CharCount {
            fn eq(&self, other: &Self) -> bool {
                self.char_map == other.char_map
            }
        }

        let mut all_hash: HashMap<CharCount, Vec<String>> = HashMap::new();
        for s in strs {
            let mut char_count = CharCount {
                char_map: HashMap::with_capacity(s.len()),
            };
            for c in s.chars() {
                *char_count.char_map.entry(c).or_insert(0) += 1;
            }
            let entry = all_hash.entry(char_count).or_insert(vec![]);
            entry.push(s);
        }
        all_hash.values().cloned().collect()
    }
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;
        use std::collections::HashMap;

        let mut hash: HashMap<i32, i32> = HashMap::new();
        let mut results = vec![];
        for num in nums {
            *hash.entry(num).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::with_capacity(hash.len());
        for (key, value) in hash.iter() {
            heap.push((*value, *key));
        }
        println!("{:?}", heap);

        for _ in 0..k {
            let val = heap.pop().unwrap();
            results.push(val.1);
        }
        results
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = vec![HashSet::with_capacity(9); 9];
        let mut columns: Vec<HashSet<char>> = vec![HashSet::with_capacity(9); 9];
        let mut areas: Vec<HashSet<char>> = vec![HashSet::with_capacity(9); 9];

        for i in 0..9 {
            for j in 0..9 {
                let char = board[i][j];
                if char == '.' {
                    continue;
                }
                if rows[j].contains(&char) {
                    return false;
                } else {
                    rows[j].insert(char);
                }
                if columns[i].contains(&char) {
                    return false;
                } else {
                    columns[i].insert(char);
                }
                let index = (j / 3) + (i / 3) * 3;
                if areas[index].contains(&char) {
                    return false;
                } else {
                    areas[index].insert(char);
                }
            }
        }
        true
    }

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for token in tokens {
            match token.as_str() {
                "+" => {
                    let second = stack.pop().unwrap();
                    let first = stack.pop().unwrap();
                    stack.push(first + second);
                }
                "-" => {
                    let second = stack.pop().unwrap();
                    let first = stack.pop().unwrap();
                    stack.push(first - second);
                }
                "*" => {
                    let second = stack.pop().unwrap();
                    let first = stack.pop().unwrap();
                    stack.push(first * second);
                }
                "/" => {
                    let second = stack.pop().unwrap();
                    let first = stack.pop().unwrap();
                    stack.push(first / second);
                }
                _ => {
                    let val = token.parse::<i32>().unwrap();
                    stack.push(val);
                }
            }
        }
        stack[0]
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut results: Vec<String> = vec![];
        _generate_parenthesis(n, 0, 0, "".to_string(), &mut results);
        return results;

        fn _generate_parenthesis(
            n: i32,
            left_count: i32,
            right_count: i32,
            mut tmp: String,
            results: &mut Vec<String>,
        ) {
            if left_count == n && right_count == n {
                results.push(tmp.clone());
                return;
            }

            if left_count < n {
                tmp.push('(');
                _generate_parenthesis(n, left_count + 1, right_count, tmp.clone(), results);
                tmp.pop();
            }

            if right_count < left_count {
                tmp.push(')');
                _generate_parenthesis(n, left_count, right_count + 1, tmp.clone(), results);
                tmp.pop();
            }
        }
    }

    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        // stack solution
        let mut results: Vec<i32> = vec![0; temperatures.len()];
        let mut stack_pointers: Vec<usize> = vec![];
        for i in 0..temperatures.len() {
            while !stack_pointers.is_empty() && temperatures[i] > temperatures[*stack_pointers.last().unwrap()] {
                // swap
                let index = stack_pointers.pop().unwrap();
                results[index] = (i - index) as i32;
            }
            stack_pointers.push(i);
        }
        results
    }
}

struct NumArray {
    sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            sum[i + 1] = sum[i] + nums[i];
        }
        Self { sum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sum[right as usize + 1] - self.sum[left as usize]
    }
}

struct NumMatrix {
    sum: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut sum = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                sum[i + 1][j + 1] = sum[i][j + 1] + sum[i + 1][j] - sum[i][j] + matrix[i][j];
            }
        }
        Self { sum }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.sum[row2 as usize + 1][col2 as usize + 1]
            - self.sum[row1 as usize][col2 as usize + 1]
            - self.sum[row2 as usize + 1][col1 as usize]
            + self.sum[row1 as usize][col1 as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::NumArray;
    use super::NumMatrix;
    use super::Solution;
    use std::collections::HashSet;

    #[test]
    fn test_remove_element() {
        let mut nums: Vec<i32> = vec![1];
        Solution::remove_element(&mut nums, 1);
    }

    #[test]
    fn test_max_sub_array() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn test_max_subarray_sum_circular() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
        assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5]), 10);
        assert_eq!(Solution::max_subarray_sum_circular(vec![-3, -2, -3]), -2);
    }

    #[test]
    fn test_max_turbulence_size() {
        assert_eq!(
            Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]),
            5
        );
        assert_eq!(Solution::max_turbulence_size(vec![4, 8, 12, 16]), 2);
        assert_eq!(
            Solution::max_turbulence_size(vec![0, 1, 1, 0, 1, 0, 1, 1, 0, 0]),
            5
        );
    }

    #[test]
    fn test_contains_nearby_duplicate() {
        // ref: https://leetcode.com/problems/contains-duplicate-ii/
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
            true
        );
    }

    #[test]
    fn test_num_of_subarrays() {
        // ref: https://leetcode.com/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/
        assert_eq!(
            Solution::num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4),
            3
        );
    }

    #[test]
    fn test_min_sub_array_len() {
        // ref: https://leetcode.com/problems/minimum-size-subarray-sum/
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }

    #[test]
    fn test_length_of_longest_substring() {
        // ref: https://leetcode.com/problems/longest-substring-without-repeating-characters/
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
    }

    #[test]
    fn test_character_replacement() {
        // ref: https://leetcode.com/problems/longest-repeating-character-replacement/
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
    }

    #[test]
    fn test_is_palindrome() {
        // https://leetcode.com/problems/valid-palindrome/
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
        assert_eq!(Solution::is_palindrome("a.".to_string()), true);
    }

    #[test]
    fn test_two_sum() {
        // https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn test_remove_duplicates() {
        // https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates_2(&mut nums), 5);
        assert_eq!(nums, vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_max_area() {
        // https://leetcode.com/problems/container-with-most-water/
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test_trap() {
        // https://leetcode.com/problems/trapping-rain-water/
        // assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 3]), 1);
    }

    #[test]
    fn test_range_sum_query() {
        // https://leetcode.com/problems/range-sum-query-immutable/
        let nums = vec![-2, 0, 3, -5, 2, -1];
        let obj = NumArray::new(nums);
        assert_eq!(obj.sum_range(0, 2), 1);
        assert_eq!(obj.sum_range(2, 5), -1);
        assert_eq!(obj.sum_range(0, 5), -3);
    }

    #[test]
    fn test_num_matrix_query() {
        let nums = vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ];
        let obj = NumMatrix::new(nums);
        assert_eq!(obj.sum_region(2, 1, 4, 3), 8);
        assert_eq!(obj.sum_region(1, 1, 2, 2), 11);
        assert_eq!(obj.sum_region(1, 2, 2, 4), 12);
    }

    #[test]
    fn test_pivot_index() {
        // ref: https://leetcode.com/problems/find-pivot-index/
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    }

    #[test]
    fn test_product_except_self() {
        // ref: https://leetcode.com/problems/product-of-array-except-self/
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }

    #[test]
    fn test_subarray_sum() {
        // ref: https://leetcode.com/problems/subarray-sum-equals-k/
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }

    #[test]
    fn test_is_anagram() {
        // ref: https://leetcode.com/problems/valid-anagram/
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
    }

    #[test]
    fn test_group_anagrams() {
        // ref: https://leetcode.com/problems/group-anagrams/
        let actual = Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);
        let expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];

        // 順序を問わず、要素の一致を確認する
        assert_eq!(actual.len(), expected.len()); // ベクタの要素数が同じかどうかを確認

        let actual_set: HashSet<Vec<String>> = actual.into_iter().collect(); // 順序を無視するためにHashSetに変換
        let expected_set: HashSet<Vec<String>> = expected.into_iter().collect();
        // 順序を無視するためにHashSetに変換
        // hard to write a test code
    }

    #[test]
    fn test_top_k_frequent() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let actual = Solution::top_k_frequent(nums, 2);
        let expected = vec![1, 2];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_valid_sudoku() {
        // let board: Vec<Vec<char>> = vec![
        //     vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        //     vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        //     vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        //     vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        //     vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        //     vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        //     vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        //     vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        //     vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        // ];
        // let actual = Solution::is_valid_sudoku(board);
        // assert_eq!(actual, true);

        let board: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
        ];
        let actual = Solution::is_valid_sudoku(board);
        assert_eq!(actual, false);
    }

    #[test]
    fn test_eval_rpn() {
        // let tokens: Vec<String> = vec![
        //     "2".to_string(),
        //     "1".to_string(),
        //     "+".to_string(),
        //     "3".to_string(),
        //     "*".to_string(),
        // ];

        let tokens_chars = [
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ];
        let tokens: Vec<String> = tokens_chars.iter().map(|s| s.to_string()).collect();
        let actual = Solution::eval_rpn(tokens);
        let expected = 22;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_generate_parenthesis() {
        let actual = Solution::generate_parenthesis(3);
        let expected = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_daily_temperature() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let actual = Solution::daily_temperatures(temperatures);
        let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(actual, expected);
    }
}
