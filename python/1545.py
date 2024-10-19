def rev(b):
    ret = ""
    for i in b:
        if i == '0':
            ret += '1'
        else:
            ret += '0'
    return ret[::-1]

class Solution:
    def findKthBit(self, n: int, k: int) -> str:
        s = []
        for i in range(n):
            if i == 0:
                s.append("0")
                continue
            s.append(s[i-1] + "1" + rev(s[i-1]))
        return s[len(s)-1][k-1]
