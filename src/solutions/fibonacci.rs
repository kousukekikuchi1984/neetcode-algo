struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        fn _fib(n: i32, cache: &mut Vec<i32>) -> i32 {
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
                        cache[nusize] = val;
                        return val;
                    }
                },
            }
        }

        let mut cache: Vec<i32> = vec![0; (n + 1) as usize];
        _fib(n, &mut cache)
    }
    pub fn climb_stairs(n: i32) -> i32 {
        fn _climb_stairs(n: i32, cache: &mut Vec<i32>) -> i32 {
            let nusize = n as usize;
            match n {
                0 => {
                    cache[nusize] = 0;
                    return 1;
                }
                1 => {
                    cache[nusize] = 1;
                    return 1;
                }
                _ => match cache[nusize] != 0 {
                    true => cache[nusize],
                    false => {
                        let val = _climb_stairs(n - 1, cache) + _climb_stairs(n - 2, cache);
                        cache[nusize] = val;
                        return val;
                    }
                },
            }
        }

        let mut cache: Vec<i32> = vec![0; (n + 1) as usize];
        _climb_stairs(n, &mut cache)
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn test_fibonacci() {
        let actual = Solution::fib(2);
        let expected = 1;
        assert_eq!(actual, expected);
        let actual = Solution::fib(5);
        let expected = 5;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_climb_stairs() {
        let actual = Solution::climb_stairs(2);
        let expected = 2;
        assert_eq!(actual, expected);
        let actual = Solution::climb_stairs(3);
        let expected = 3;
        assert_eq!(actual, expected);
        let actual = Solution::climb_stairs(4);
        let expected = 5;
        assert_eq!(actual, expected);
    }
}
