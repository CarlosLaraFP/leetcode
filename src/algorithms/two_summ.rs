/*
    Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
    You may assume that each input would have exactly one solution, and you may not use the same element twice.
    You can return the answer in any order.

    Example 1:

    Input: nums = [2,7,11,15], target = 9
    Output: [0,1]
    Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
    Example 2:

    Input: nums = [3,2,4], target = 6
    Output: [1,2]
    Example 3:

    Input: nums = [3,3], target = 6
    Output: [0,1]
    
    Constraints:

    2 <= nums.length <= 104
    -109 <= nums[i] <= 109
    -109 <= target <= 109
    Only one valid answer exists.
    
    Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?
 */
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // key: number, value: index
    let mut map: HashMap<i32, i32> = HashMap::new(); // with capacity is also an option
    let mut result = Vec::with_capacity(2);

    // The target - current value = must be a key in the map

    for (i, n) in nums.iter().enumerate() {
        let x = target - n;

        if let Some(index) = map.get(&x) {
            result.extend([*index, i as i32]);
            break;
        } else {
            map.insert(*n, i as i32);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(vec![0, 1], two_sum(nums, 9));
    }

    #[test]
    fn second() {
        let nums = vec![3, 2, 4];
        assert_eq!(vec![1, 2], two_sum(nums, 6));
    }

    #[test]
    fn third() {
        let nums = vec![3, 3];
        assert_eq!(vec![0, 1], two_sum(nums, 6));
    }
}
