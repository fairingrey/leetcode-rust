// https://leetcode.com/problems/median-of-two-sorted-arrays/
pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(a: Vec<i32>, b: Vec<i32>) -> f64 {
        // Here, we shadow 'a' and 'b' as slices referencing their corresponding vectors
        // Note that the following algorithm works on the assumption that `a` is at most as long as `b` if not shorter
        // if that's not the case, we assign the two references such that it is
        let (a, b) = if a.len() < b.len() {
            (&a[..], &b[..])
        } else {
            (&b[..], &a[..])
        };

        // `left` and `right` are indices which indicate where an ideal partitioning index for `a` can exist
        // such that all elements of `a[..part_a] + b[..part_b]` are less than `a[part_a..] + b[part_b..]`
        let mut left = 0;
        let mut right = a.len();

        while left <= right {
            // partitioning indices for slices `a` and `b`
            let part_a = (left + right) / 2;
            let part_b = (a.len() + b.len() + 1) / 2 - part_a;

            let max_left_a = if part_a == 0 {
                std::i32::MIN
            } else {
                a[part_a - 1]
            };
            let min_right_a = if part_a == a.len() {
                std::i32::MAX
            } else {
                a[part_a]
            };

            let max_left_b = if part_b == 0 {
                std::i32::MIN
            } else {
                b[part_b - 1]
            };
            let min_right_b = if part_b == b.len() {
                std::i32::MAX
            } else {
                b[part_b]
            };

            if max_left_a > min_right_b {
                // if `max_left_a > min_right_b`, search further to the left of the current partitioning index
                right = part_a - 1;
            } else if max_left_b > min_right_a {
                // if `max_left_b > min_right_a`, search further to the right of the current partitioning index
                left = part_a + 1;
            } else {
                // The partitioning indices for slices `a` and `b` are found such that
                // all elements of `a[..part_a] + b[..part_b]` are less than `a[part_a..] + b[part_b..]`
                // Thus, we can determine the median here.
                if (a.len() + b.len()) % 2 == 1 {
                    return f64::from(max_left_a.max(max_left_b));
                } else {
                    return f64::from(max_left_a.max(max_left_b) + min_right_a.min(min_right_b))
                        / 2f64;
                }
            }
        }

        panic!("This code should not be reachable normally -- were unsorted arrays provided?")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: implementation
    #[test]
    fn n0004_1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
