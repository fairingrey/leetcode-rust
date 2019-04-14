// https://leetcode.com/problems/edit-distance/

pub struct Solution {}

use std::cmp::min;
use std::mem::replace;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // these arrays hold the cost values for transforming word1 into word2
        let mut prev_row = vec![0; word1.len() + 1];
        let mut cur_row = vec![0; word1.len() + 1];

        for i in 0..prev_row.len() {
            prev_row[i] = i;
        }

        let mut nword1 = String::from("_");
        nword1.push_str(&word1);
        let mut nword2 = String::from("_");
        nword2.push_str(&word2);

        for (j, char2) in nword2.chars().enumerate() {
            cur_row[0] = j;
            for (i, char1) in nword1.chars().enumerate().skip(1) {
                if char1 == char2 {
                    cur_row[i] = prev_row[i - 1];
                } else {
                    cur_row[i] = min(
                        cur_row[i - 1] + 1,
                        min(prev_row[i] + 1, prev_row[i - 1] + 1),
                    );
                }
            }
            replace(&mut prev_row, cur_row.clone());
        }

        *cur_row.last().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn n0072_1() {
        assert_eq!(
            3,
            Solution::min_distance(String::from("horse"), String::from("ros"))
        );
    }

    #[test]
    pub fn n0072_2() {
        assert_eq!(
            3,
            Solution::min_distance(String::from("kitten"), String::from("sitting"))
        );
    }

    #[test]
    pub fn n0072_3() {
        assert_eq!(
            1,
            Solution::min_distance(String::from(""), String::from("a"))
        );
    }
}
