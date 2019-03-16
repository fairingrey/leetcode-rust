// https://leetcode.com/problems/jewels-and-stones/

pub struct Solution {}

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut count: i32 = 0;

        for c in s.chars() {
            for d in j.chars() {
                if c == d { count += 1; }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n0771_1() {
        assert_eq!(3, Solution::num_jewels_in_stones("aA".to_owned(), "aAAbbbb".to_owned()));
        assert_eq!(0, Solution::num_jewels_in_stones("z".to_owned(), "ZZ".to_owned()));
    }
}
