//! # Three Sum
//!
//! Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
//!
//! Notice that the solution set must not contain duplicate triplets.

#![allow(unused)]

use std::collections::{HashMap, HashSet};
struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // There are a few different ways to solve this problem, but let's solve it
        // using a hashset. At each number, we'll
        let mut seen_map: HashMap<i32, i32> = HashMap::new();
        let mut dup_set: HashSet<i32> = HashSet::new();
        let mut ret: HashSet<Vec<i32>> = HashSet::new();

        nums.iter().enumerate().for_each(|(i, &n)| {
            if !dup_set.contains(&n) {
                dup_set.insert(n);
                nums[i + 1..].iter().enumerate().for_each(|(j, p)| {
                    let complement = -n - p; //(n + *p) * -1;
                    if seen_map.contains_key(&complement)
                        && seen_map.get(&complement).unwrap().eq(&(i as i32))
                    {
                        let mut new_entry = vec![n, *p, complement];
                        new_entry.sort();
                        ret.insert(new_entry);
                    }
                    seen_map.insert(*p, i as i32);
                });
            }
        });

        let mut res = ret.into_iter().collect::<Vec<Vec<i32>>>();
        res.sort();
        return res;
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
