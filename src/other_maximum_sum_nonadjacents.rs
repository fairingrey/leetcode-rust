//    Not a leetcode problem (yet?), hence the 'other'. I'm just including it here for kicks

//    This problem was asked by Airbnb.
//
//    Given a list of integers, write a function that returns the largest sum of non-adjacent numbers. Numbers can be 0 or negative.
//
//    For example, [2, 4, 6, 2, 5] should return 13, since we pick 2, 6, and 5. [5, 1, 1, 5] should return 10, since we pick 5 and 5.
//
//    Follow-up: Can you do this in O(N) time and constant space?

pub struct Solution {}
impl Solution {
    pub fn maximum_sum_nonadjacents(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut sum_include_prev = nums[0];
        let mut sum_exclude_prev = 0;

        let iterator = nums.iter().skip(1);

        for num in iterator {
            if sum_exclude_prev + *num >= sum_include_prev {
                sum_exclude_prev += *num;
            } else {
                sum_exclude_prev = sum_include_prev;
            }
            std::mem::swap(&mut sum_exclude_prev, &mut sum_include_prev);
        }

        std::cmp::max(sum_include_prev, sum_exclude_prev)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn other_nonadjacents_1() {
        assert_eq!(13, Solution::maximum_sum_nonadjacents(vec![2, 4, 6, 2, 5]));
    }

    #[test]
    fn other_nonadjacents_2() {
        assert_eq!(10, Solution::maximum_sum_nonadjacents(vec![5, 1, 1, 5]));
    }
}
