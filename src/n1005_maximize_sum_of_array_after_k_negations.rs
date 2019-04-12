// https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/

pub struct Solution {}

impl Solution {
    pub fn largest_sum_after_k_negations(mut a: Vec<i32>, k: i32) -> i32 {
        a.sort_unstable();
        for _ in 0..k {
            a[0] *= -1;
            a.sort_unstable();
        }
        a.iter().sum()
    }
}
