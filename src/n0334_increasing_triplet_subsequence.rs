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

    // This is an alternate solution that works (and also generalizes the length of the subsequence
    pub fn increasing_subsequence_generalized(nums: Vec<i32>) -> bool {
        let subseq_length = 3;

        let mut subseq_nums = vec![std::i32::MAX; subseq_length - 1];

        for v in nums.into_iter() {
            for subseq_v in &mut subseq_nums {
                if v <= *subseq_v {
                    *subseq_v = v;
                    break;
                }
            }
            if v > *subseq_nums.last().unwrap() {
                return true;
            }
        }
        false
    }
}
