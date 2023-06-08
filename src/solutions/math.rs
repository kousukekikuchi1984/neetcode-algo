struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        let mut digits = digits.clone();

        digits = digits
            .into_iter()
            .rev()
            .map(|digit| {
                let val = digit + carry;
                carry = val / 10;
                (val % 10)
            })
            .collect();
        digits.reverse();
        if carry == 1 {
            digits.insert(0, 1);
        }
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }
}
