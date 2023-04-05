struct Solution {}

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut results = 0;
        let mut n = n;
        while n > 0 {
            if n & 1 == 1 {
                results += 1;
            }
            n >>= 1;
        }
        results
    }

    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut results = vec![0; n as usize + 1];
        for i in 1..=n {
            results[i as usize] = results[i as usize >> 1] + (i & 1);
        }
        results
    }

    pub fn reverse_bits(x: u32) -> u32 {
        let mut result: u32 = 0;
        let mut x = x;
        for _ in 0..32 {
            result <<= 1;
            result += x & 1;
            x >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        assert_eq!(
            Solution::hammingWeight(0b00000000000000000000000000001011),
            3
        );
        assert_eq!(
            Solution::hammingWeight(0b00000000000000000000000010000000),
            1
        );
    }

    #[test]
    fn test_count_bits() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn test_reverse_bits() {
        // ref: https://leetcode.com/problems/reverse-bits/
        assert_eq!(
            Solution::reverse_bits(0b00000010100101000001111010011100),
            0b00111001011110000010100101000000
        );
    }
}
