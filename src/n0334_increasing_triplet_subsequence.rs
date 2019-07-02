// https://leetcode.com/problems/increasing-triplet-subsequence/

pub struct Solution {}
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut v1 = std::i32::MAX;
        let mut v2 = std::i32::MAX;

        for v in nums.into_iter() {
            if v <= v1 {
                v1 = v;
            } else if v <= v2 {
                v2 = v;
            } else {
                return true;
            }
        }

        false
    }
}
