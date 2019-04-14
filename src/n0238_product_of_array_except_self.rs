// https://leetcode.com/problems/product-of-array-except-self/

pub struct Solution {}
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut products: Vec<i32> = vec![1; nums.len()];
        let mut left: i32 = 1;
        let mut right: i32 = 1;

        for (num, product) in nums.iter().zip(products.iter_mut()) {
            *product *= left;
            left *= num;
        }

        for (num, product) in nums.iter().zip(products.iter_mut()).rev() {
            *product *= right;
            right *= num;
        }

        products
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
