// https://leetcode.com/problems/longest-substring-with-at-most-k-distinct-characters

// Note this is a subscribe problem

//    Good morning! Here's your coding interview problem for today.
//
//    This problem was asked by Amazon.
//
//    Given an integer k and a string s, find the length of the longest substring that contains at most k distinct characters.
//
//    For example, given s = "abcba" and k = 2, the longest substring with k distinct characters is "bcb".
use std::collections::HashMap;
pub struct Solution {}
impl Solution {
    pub fn longest_substr_with_at_most_k_distinct_chars(s: String, k: usize) -> String {
        // this is a sliding window algorithm
        if s.is_empty() || k == 0 {
            return String::from("");
        }

        // keys are chars, values are indices when first seen
        let mut seen_chars = HashMap::new();
        let mut earliest_seen_char = s.chars().nth(0).unwrap() as u32;
        let mut longest_substr_start = 0;
        let mut longest_substr_end = 0;

        let mut start = 0;

        for (i, c) in s.chars().enumerate() {
            let c = c as u32;
            if seen_chars.contains_key(&c) || seen_chars.len() < k {
                seen_chars.entry(c).or_insert(i);
            } else if seen_chars.len() >= k {
                // find where the next character that isn't the last earliest seen char is
                let index_earliest_seen_char = seen_chars.remove(&earliest_seen_char).unwrap();
                let rest_of_str = s.get(index_earliest_seen_char + 1..=i).unwrap();
                if let Some(j) = rest_of_str
                    .chars()
                    .position(|x| x as u32 != earliest_seen_char)
                {
                    start = index_earliest_seen_char + 1 + j;
                    earliest_seen_char = s.chars().nth(start).unwrap() as u32;
                    seen_chars.insert(c, i);
                }
            }
            if i - start > longest_substr_end - longest_substr_start {
                longest_substr_start = start;
                longest_substr_end = i;
            }
        }

        s.get(longest_substr_start..=longest_substr_end)
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n0340_1() {
        assert_eq!(
            String::from("bcb"),
            Solution::longest_substr_with_at_most_k_distinct_chars(String::from("abcba"), 2)
        );
    }
    #[test]
    fn n0340_2() {
        assert_eq!(
            String::from("aaaa"),
            Solution::longest_substr_with_at_most_k_distinct_chars(String::from("aabaaaabaaa"), 1)
        );
    }
    #[test]
    fn n0340_3() {
        assert_eq!(
            String::from("aaaaa"),
            Solution::longest_substr_with_at_most_k_distinct_chars(String::from("aaaaabbcaaaaa"), 1)
        );
    }
    #[test]
    fn n0340_4() {
        assert_eq!(
            String::from("cadcacacaca"),
            Solution::longest_substr_with_at_most_k_distinct_chars(String::from("abcadcacacaca"), 3)
        );
    }
}
