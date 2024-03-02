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

    Space complexity: O(N)
    Time complexity: O(N)

 */
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    // nums is moved into the function by the caller

    let n = nums.len();

    let mut result = vec![0; n];
    let (mut left, mut right) = (0 as usize, n - 1);
    let mut index = n - 1;

    while left <= right { // the overlap is the last element of nums
        if nums[left].abs() >= nums[right].abs() {
            // left goes at the end, which is the current index
            result[index] = nums[left].pow(2);
            left += 1;
        }
        else {
            result[index] = nums[right].pow(2);
            right -= 1;
        }
        // prevents unsigned integer type usize from becoming negative at the end
        if index > 0 {
            index -= 1;
        }

        //println!("{:?}", result);
    }
    
    result // moved out of the function into the caller
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let nums = vec![-4, 0, 1, 3, 8];
        assert_eq!(vec![0, 1, 9, 16, 64], sorted_squares(nums));
    }

    #[test]
    fn second() {
        let nums = vec![-7,-3,2,3,11];
        assert_eq!(vec![4,9,9,49,121], sorted_squares(nums));
    }

    #[test]
    fn third() {
        let nums = vec![2];
        assert_eq!(vec![4], sorted_squares(nums));
    }
}
