from math import log2
class Solution:
    def isPowerOfTwo(self, n: int) -> bool:
        if n <= 0:
            return False
        if n == (2 ** round(log2(n))):
            return True
        return False
