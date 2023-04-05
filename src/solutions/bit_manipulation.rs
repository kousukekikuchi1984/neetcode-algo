struct Solution {}

impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight () {
        assert_eq!(Solution::hammingWeight(0b00000000000000000000000000001011), 3);
        assert_eq!(Solution::hammingWeight(0b00000000000000000000000010000000), 1);
    }
}