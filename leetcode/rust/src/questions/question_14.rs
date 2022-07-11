pub struct Solution {}

impl Solution {
    /*
    Write a function to find the longest common prefix string amongst an array of strings.

    If there is no common prefix, return an empty string "".



    Example 1:

    Input: strs = ["flower","flow","flight"]
    Output: "fl"
    Example 2:

    Input: strs = ["dog","racecar","car"]
    Output: ""
    Explanation: There is no common prefix among the input strings.


    Constraints:

    1 <= strs.length <= 200
    0 <= strs[i].length <= 200
    strs[i] consists of only lowercase English letters.
    */
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs.first().unwrap().to_string();
        }

        let mut sorted = strs.clone();

        sorted.sort();

        let mut longest_prefix = "".to_string();

        let head_tail = sorted.split_at(1);

        let head = head_tail.0.first().unwrap();

        let tail = head_tail.1;

        let tail_length = tail.len();

        for i in 1..head.len() + 1
        {
            let mut found_count = 0;

            let prefix = head.split_at(i).0.to_string();

            for string in tail {
                if string.starts_with(&prefix) {
                    found_count += 1
                }
            }

            if found_count == tail_length
            {
                longest_prefix = prefix;
            }
        }

        return longest_prefix.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_prefix_is_found() {
        assert_eq!(
            Solution::longest_common_prefix(
                vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]
            ),
            "fl".to_string());
    }

    #[test]
    fn test_no_common_prefix_is_found() {
        assert_eq!(
            Solution::longest_common_prefix(
                vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]),
            "".to_string()
        );
    }
}

