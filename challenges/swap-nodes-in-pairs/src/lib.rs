//! # Three Sum
//!
//! Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
//!
//! Notice that the solution set must not contain duplicate triplets.

#![allow(unused)]

use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => head,
            // Update `first_node` as mutable
            Some(mut first_node) => {
                match first_node.next {
                    None => Some(first_node),
                    // Update `next_node` as mutable
                    Some(mut next_node) => {
                        // Update pointers
                        // Get the next linked nodes
                        first_node.next = Self::swap_pairs(next_node.next);
                        // Take the next node and link it's `next` to
                        // the node we're currently on:
                        next_node.next = Some(first_node);
                        Some(next_node)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::linked_list;

    use super::*;

    #[test]
    fn test_challenge() {
        let head = setup_linked_list(vec![1, 2, 3, 4]);
        let expected = setup_linked_list(vec![2, 1, 4, 3]);

        assert_eq!(Solution::swap_pairs(head), expected);
    }

    #[test]
    fn test_challenge_two() {
        let head = setup_linked_list(vec![1]);
        let expected = setup_linked_list(vec![1]);

        assert_eq!(Solution::swap_pairs(head), expected);
    }

    #[test]
    fn test_challenge_three() {
        let head = setup_linked_list(vec![2, 1]);
        let expected = setup_linked_list(vec![1, 2]);

        assert_eq!(Solution::swap_pairs(head), expected);
    }

    fn setup_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;

        for val in values.into_iter() {
            let node = Box::new(ListNode::new(val));
            *tail = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }

        head
    }
}
