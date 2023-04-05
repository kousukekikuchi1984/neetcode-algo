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
}
