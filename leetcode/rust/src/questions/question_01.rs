use std::collections::HashMap;

pub struct Solution {}

impl Solution {
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
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut matching_numbers: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len()
        {
            let number = nums[i];

            if matching_numbers.contains_key(&number)
            {
                return Vec::from([matching_numbers[&number], i as i32]);
            } else {
                matching_numbers.insert(target - number, i as i32);
            }
        }

        return Vec::from([-1, -1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sums_first_two_indices() {
        let expected = vec![0, 1];
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);

        assert_eq!(expected.len(), result.len());

        for i in 0..expected.len() {
            assert_eq!(expected[i], result[i]);
        }
    }

    #[test]
    fn test_two_sums_last_two_indices() {
        let expected = vec![1, 2];
        let result = Solution::two_sum(vec![3, 2, 4], 6);

        assert_eq!(expected.len(), result.len());

        for i in 0..expected.len() {
            assert_eq!(expected[i], result[i]);
        }
    }

    #[test]
    fn test_two_sums_first_and_last_indices() {
        let expected = vec![0, 1];
        let result = Solution::two_sum(vec![3, 3], 6);

        assert_eq!(expected.len(), result.len());

        for i in 0..expected.len() {
            assert_eq!(expected[i], result[i]);
        }
    }
}
