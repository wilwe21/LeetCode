class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        l = []
        for i in nums:
            if i not in l:
                l.append(i)
        nums.clear()
        for i in l:
            nums.append(i)
        return len(nums)
