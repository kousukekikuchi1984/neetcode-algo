struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        fn _fib(n: i32, cache: &mut Vec<i32>) -> i32 {
            println!("cache: {:?}", cache);
            let nusize = n as usize;
            match n {
                0 => {
                    cache[nusize] = 0;
                    return 0;
                }
                1 => {
                    cache[nusize] = 1;
                    return 1;
                }
                _ => match cache[nusize] != 0 {
                    true => cache[nusize],
                    false => {
                        let val = _fib(n - 1, cache) + _fib(n - 2, cache);
                        println!("val: {}", val);
                        cache[nusize] = val;
                        return val;
                    }
                },
            }
        }

        let mut cache: Vec<i32> = vec![0; (n + 1) as usize];
        _fib(n, &mut cache)
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn test_fibonacci() {
        let actual = Solution::fib(2);
        let expected = 2;
        assert_eq!(actual, expected);
        let actual = Solution::fib(5);
        let expected = 8;
        assert_eq!(actual, expected);
    }
}
