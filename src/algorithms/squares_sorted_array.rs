/*
    Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.

    Example 1:

    Input: nums = [-4,-1,0,3,10]
    Output: [0,1,9,16,100]
    Explanation: After squaring, the array becomes [16,1,0,9,100].
    After sorting, it becomes [0,1,9,16,100].
    Example 2:

    Input: nums = [-7,-3,2,3,11]
    Output: [4,9,9,49,121]

    Squaring each element and sorting the new array is very trivial, could you find an O(n) solution using a different approach?

    Solution:

    Space complexity: O(1)
    Time complexity: O(N)

 */
pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
    // nums is moved into the function by the caller
    nums.iter_mut().for_each(|n| *n = n.pow(2));

    nums.sort(); 
    
    nums // moved out of the function into the caller
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let nums = vec![-4, 0, 1, 3, 8];
        assert_eq!(vec![0, 1, 9, 16, 64], sorted_squares(nums));
    }
}
