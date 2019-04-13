// https://leetcode.com/problems/3sum/

pub struct Solution {}
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut results: Vec<Vec<i32>> = Vec::new();

        if nums.len() < 3 {
            return results;
        }

        for (index, a) in nums[0..nums.len() - 2].iter().enumerate() {
            let a = *a;
            let mut start = index + 1;
            let mut end = nums.len() - 1;

            while start < end {
                let b = nums[start];
                let c = nums[end];
                let sum = a + b + c;

                if sum == 0 {
                    results.push(vec![a, b, c]);
                    start += 1;
                    end -= 1;
                } else if sum > 0 {
                    end -= 1;
                } else {
                    start += 1;
                }
            }
        }

        results.sort_unstable();
        results.dedup();

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n0015_1() {
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
    }
}
