// https://leetcode.com/problems/product-of-array-except-self/

pub struct Solution {}
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res_vec: Vec<i32> = Vec::new();

        for (index, _num) in nums.iter().enumerate() {
            let product = nums
                .get(0..index)
                .expect("oops")
                .iter()
                .fold(1, |acc, x| acc * x)
                * nums
                    .get(index + 1..nums.len())
                    .expect("oops")
                    .iter()
                    .fold(1, |acc, x| acc * x);
            res_vec.push(product);
        }

        res_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n0238_1() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
    }
}
