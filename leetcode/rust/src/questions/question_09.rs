pub struct Solution {}

impl Solution {
    /*
    9. Palindrome Number
    Easy

    6323

    2196

    Add to List

    Share
    Given an integer x, return true if x is palindrome integer.

    An integer is a palindrome when it reads the same backward as forward.

    For example, 121 is a palindrome while 123 is not.


    Example 1:

    Input: x = 121
    Output: true
    Explanation: 121 reads as 121 from left to right and from right to left.
    Example 2:

    Input: x = -121
    Output: false
    Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
    Example 3:

    Input: x = 10
    Output: false
    Explanation: Reads 01 from right to left. Therefore it is not a palindrome.


    Constraints:

    -231 <= x <= 231 - 1


    Follow up: Could you solve it without converting the integer to a string?
    */
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() { return false; }

        x.to_string().chars().rev().collect::<String>() == x.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_palindrome_is_true() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn test_negative_palindrome_is_false() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn test_positive_palindrome_is_false() {
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
