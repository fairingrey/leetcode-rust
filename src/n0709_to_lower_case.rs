// https://leetcode.com/problems/to-lower-case/

pub struct Solution {}

impl Solution {
    pub fn to_lower_case(mut str: String) -> String {
        str.make_ascii_lowercase();
        str
    }
}
