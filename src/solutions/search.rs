use std::cmp::Ordering;

struct Solution {}

fn guess(num: i32) -> i32 {
    if num > 1150769282 {
        return -1;
    }

    if num < 1150769282 {
        return 1;
    }

    0
}

fn isBadVersion(version: i32) -> bool {
    version >= 1150769282
}

impl Solution {
    pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        fn _search(nums: &[i32], left: isize, right: isize, target: i32) -> isize {
            if left > right {
                return -1;
            }
            let middle = left + (right - left) / 2;
            match target.cmp(&nums[middle as usize]) {
                Ordering::Less => _search(nums, left, middle - 1, target),
                Ordering::Equal => middle,
                Ordering::Greater => _search(nums, middle + 1, right, target),
            }
        }

        let length = nums.len() - 1;
        let num = _search(&nums, 0, length.try_into().unwrap(), target);
        return num as i32;
    }

    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        fn _search_column(matrix: &[&[i32]], bottom: isize, top: isize, target: i32) -> isize {
            let row = (top + bottom) / 2;
            let last = matrix[0].len() - 1;

            if bottom > top || bottom < 0 || top > (matrix.len() - 1) as isize {
                return -1;
            }
            if matrix[row as usize][last] < target {
                return _search_column(matrix, row + 1, top, target);
            } else if matrix[row as usize][0] > target {
                return _search_column(matrix, bottom, row - 1, target);
            } else {
                return row;
            }
        }

        fn _search_rows(rows: &[i32], left: isize, right: isize, target: i32) -> bool {
            if left > right {
                return false;
            }
            let middle = (left + right) / 2;
            match target.cmp(&rows[middle as usize]) {
                Ordering::Less => _search_rows(rows, left, middle - 1, target),
                Ordering::Equal => true,
                Ordering::Greater => _search_rows(rows, middle + 1, right, target),
            }
        }
        let rows: Vec<&[i32]> = matrix.iter().map(|v| v.as_slice()).collect();
        let row = _search_column(rows.as_slice(), 0, (matrix.len() - 1) as isize, target);
        if row == -1 {
            return false;
        }
        let rows = &matrix[row as usize];
        _search_rows(&rows, 0, (rows.len() - 1) as isize, target)
    }

    unsafe fn guessNumber(n: i32) -> i32 {
        let mut start = 0;
        let mut end = n;
        while start <= end {
            let middle = start + (end - start) / 2;
            match guess(middle) {
                -1 => {
                    end = middle - 1;
                }
                0 => return middle,
                1 => {
                    start = middle + 1;
                }
                _ => panic!("unreachable"),
            };
        }
        return -1;
    }

    pub fn first_bad_version(n: i32) -> i32 {
        let mut start = 1;
        let mut end = n;
        while start < end {
            let middle = start + (end - start) / 2;
            match isBadVersion(middle) {
                true => end = middle,
                false => start = middle + 1,
            };
        }
        return start;
    }

    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        // piles の最大値を用いて、binary search をする
        // その時に規定時間いないであるかどうかを判断
        fn will_be_in_time(piles: &[i32], time: i32) -> i32 {
            let mut acc_time = 0;
            for pile in piles {
                acc_time += pile / time;
                if pile % time > 0 {
                    acc_time += 1;
                }
            }
            return acc_time;
        }

        if piles.len() == 1 {
            let mut count = 0;
            count += piles[0] / h;
            if piles[0] % h > 0 {
                count += 1;
            }
            return count;
        }

        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        let mut middle = -1;
        while left <= right {
            middle = left + (right - left) / 2;
            let time_spent = will_be_in_time(&piles, middle);
            println!("time_spent: {}, time: {}", time_spent, middle);
            match time_spent.cmp(&h) {
                Ordering::Less => right = middle - 1,
                Ordering::Equal => return middle,
                Ordering::Greater => left = middle + 1,
            }
        }
        return middle;
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
        assert_eq!(Solution::binary_search(vec![5], -5), -1);
    }

    #[test]
    fn test_search_matrix() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let actual = Solution::search_matrix(matrix, 3);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_guess() {
        unsafe {
            let actual = Solution::guessNumber(1420736637);
            assert_eq!(actual, 1150769282);
        }
    }

    #[test]
    fn test_first_bad_version() {
        let actual = Solution::first_bad_version(1420736637);
        assert_eq!(actual, 1150769282);
    }

    #[test]
    fn test_min_eating_time() {
        let actual = Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5);
        assert_eq!(actual, 30);
        let actual = Solution::min_eating_speed(vec![312884470], 312884469);
        assert_eq!(actual, 2);
    }
}
