// https://leetcode.com/problems/intersection-of-two-arrays/

pub struct Solution {}

impl Solution {
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();
        nums1.dedup();
        nums2.dedup();

        let mut list: Vec<i32> = Vec::new();

        for n in nums1.iter() {
            if nums2.contains(n) {
                list.push(*n);
            }
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n0349_1() {
        assert_eq!(
            vec![2],
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2])
        );
        assert_eq!(
            vec![4, 9],
            Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4])
        );
    }
}
