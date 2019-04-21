// https://leetcode.com/problems/longest-absolute-file-path/
use std::cmp::max;
use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut longest_len = 0;
        let mut path_lens = HashMap::new();
        path_lens.insert(0, 0);
        let lines = input.split('\n');

        for line in lines {
            let name = line.trim_start_matches(|c| c == '\t');
            let depth = line.len() - name.len();
            if name.contains('.') {
                longest_len = max(longest_len, path_lens.get(&depth).unwrap() + name.len())
            } else {
                path_lens.insert(depth + 1, path_lens.get(&depth).unwrap() + name.len() + 1);
            }
        }

        longest_len as i32
    }
}
