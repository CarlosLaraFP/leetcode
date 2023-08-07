/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
///
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
///
/// You can return the answer in any order.
///
/// Time complexity: O(N), Space complexity: O(1)
///
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() < 2 { panic!("Input array size must be > 2"); }

    let mut indices: HashMap<&i32, usize> = HashMap::with_capacity(2);

    for (i, n) in nums.iter().enumerate() {
        let difference = target - n;
        if let Some(&j) = indices.get(&difference) {
            return vec![j as i32, i as i32];
        }
        indices.insert(n, i);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let nums = vec![2, 7, 11, 15];
        let output = two_sum(nums, 9);
        assert_eq!(output, vec![0, 1]);
    }

    #[test]
    fn second() {
        let nums = vec![3, 2, 4];
        let output = two_sum(nums, 6);
        assert_eq!(output, vec![1, 2]);
    }

    #[test]
    fn third() {
        let nums = vec![3, 3];
        let output = two_sum(nums, 6);
        assert_eq!(output, vec![0, 1]);
    }

    #[test]
    fn fourth() {
        let nums = vec![-1, -2, 9];
        let output = two_sum(nums, 8);
        assert_eq!(output, vec![0, 2]);
    }
}
