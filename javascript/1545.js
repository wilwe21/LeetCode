/**
 * @param {number} n
 * @param {number} k
 * @return {character}
 */
var findKthBit = function(n, k) {
    var rev = (b) => {
        var ret = ""
        for (let i = 0; i < b.length; i++) {
            if (b[i] == "0") {
                ret += "1"
            } else {
                ret += "0"
            }
        }
        return ret.split("").reverse().join("")
    }
    var s = []
    for (let i = 0; i < n; i++) {
        if (i == 0) {
            s.push("0")
            continue
        }
        s.push(s[i-1]+"1"+rev(s[i-1]))
    }
    return s[s.length-1].split("")[k-1]
};
