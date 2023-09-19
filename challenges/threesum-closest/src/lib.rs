//! 3sum closest
//!
//! Given an integer array nums of length n and an integer target,
//! find three integers in nums such that the sum is closest to target.
//!
//! Return the sum of the three integers.
//!
//! You may assume that each input would have exactly one solution.

#![allow(unused)]

use std::collections::{HashMap, HashSet};
struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        // sort the array so we can iterate forward
        let mut sorted = nums.clone();
        sorted.sort();
        // initially start out with a super high number (infinity)
        let mut closest = i32::MAX;

        // for every number, set an index for iterating through the numbers
        // this allows us to use a 2-pointer solution
        for (curr, num) in sorted.iter().enumerate() {
            // the left pointer is the next value
            let mut left = curr + 1;
            // right is the last possible value
            let mut right = sorted.len() - 1;

            // keeping a previous difference allows us to check if we're closer to
            // the target at every iteration
            let previous_diff = sorted[curr] - target;
            // loop for each pointer
            while left < right {
                // Get the current difference
                let curr_diff: i32 = previous_diff + sorted[left] + sorted[right];
                // if the current difference is closer than the initial target
                // set the closest to the current difference
                if curr_diff.abs() < closest.abs() {
                    closest = curr_diff;
                }
                // match on the number, if it's positive, move the right pointer
                // left, if it's negative, move the left pointer forward
                // if it's zero, we can kill the entire process early
                match curr_diff.signum() {
                    1 => right -= 1,
                    -1 => left += 1,
                    0 => return target,
                    _ => unreachable!(),
                }
            }
        }

        // The closest difference will be the sum of the target
        // and the closest possible value
        return target + closest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        pub input: Vec<i32>,
        pub target: i32,
        pub expected: i32,
    }

    impl TestCase {
        pub fn assert_best(&self) {
            assert_eq!(
                Solution::three_sum_closest(self.input.clone(), self.target),
                self.expected
            )
        }
    }

    #[test]
    fn test_challenge() {
        let test_cases = vec![
            TestCase {
                input: vec![-1, 2, 1, 4],
                target: 1,
                expected: 2,
            },
            TestCase {
                input: vec![0, 0, 0],
                target: 1,
                expected: 0,
            },
        ];
        test_cases.iter().for_each(|tc| {
            tc.assert_best();
        })
        // let nums = vec![-1, 2, 1, 4];
        // let target = 1;
        // let expected = 2;

        // assert_eq!(Solution::three_sum_closest(nums, target), expected);
    }
}
