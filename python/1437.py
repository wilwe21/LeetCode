class Solution:
    def kLengthApart(self, nums: List[int], k: int) -> bool:
       st = ''.join([str(num) for num in nums])
       n = st.split("1")[1:-1]
       for i in n:
            if len(i) < k:
                return False
       return True
