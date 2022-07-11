from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        sums = {}
        for index, num in enumerate(nums):
            if num in sums:
                return [sums[num], index]
            else:
                sums[target - num] = index

    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False

        x_as_string = str(x)

        return x_as_string == x_as_string[::-1]
