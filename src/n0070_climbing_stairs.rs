// https://leetcode.com/problems/climbing-stairs/

pub struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 || n == 2 {
            return n;
        }

        let mut a = 1;
        let mut b = 2;

        for _ in 1..n {
            let temp = a + b;
            a = b;
            b = temp;
        }

        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n0070_1() {
        assert_eq!(3, Solution::climb_stairs(3));
    }

    #[test]
    fn n0070_2() {
        assert_eq!(10946, Solution::climb_stairs(20));
    }
}
