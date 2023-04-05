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
    fn test_counting_bits() {
        assert_eq!(
            Solution::count_bits(2),
            vec![0, 1, 1]
        );
        assert_eq!(
            Solution::count_bits(5),
            vec![0, 1, 1, 2, 1, 2]
        );
    }
}
