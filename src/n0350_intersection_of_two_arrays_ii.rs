// https://leetcode.com/problems/intersection-of-two-arrays-ii/

pub struct Solution {}

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();

        let mut list: Vec<i32> = Vec::new();

        let mut i: usize = 0;
        let mut j: usize = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                list.push(nums1[i]);
                i += 1;
                j += 1;
            } else if nums1[i] < nums2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n0350_1() {
        assert_eq!(
            vec![2, 2],
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2])
        );
        assert_eq!(
            vec![4, 9],
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4])
        );
    }
}
