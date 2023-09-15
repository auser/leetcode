//! # Three Sum
//!
//! Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
//!
//! Notice that the solution set must not contain duplicate triplets.

#![allow(unused)]

use std::collections::{HashMap, HashSet};
struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        nums.sort();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    ret.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;

                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
            }
        }

        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

        assert_eq!(Solution::three_sum(nums), expected);
    }

    #[test]
    fn test_ones() {
        let nums = [0, 1, 1];
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(Solution::three_sum(nums.to_vec()), expected);
    }

    #[test]
    fn test_zeros() {
        let nums = [0, 0, 0];
        let expected: Vec<Vec<i32>> = vec![vec![0, 0, 0]];

        assert_eq!(Solution::three_sum(nums.to_vec()), expected);
    }
}
