class Solution:
    def arithmeticTriplets(self, nums: List[int], diff: int) -> int:
        c = 0
        for i in range(0, len(nums)):
            for j in range(i, len(nums)):
                for k in range(j, len(nums)):
                    if nums[j] - nums[i] == diff and nums[k] - nums[j] == diff:
                        c += 1
        return c
