/**
 * @param {number} n
 * @return {boolean}
 */
var isPowerOfTwo = function(n) {
    var c = Math.log2(n)
    if (isNaN(c) || c == -Infinity) {
        return false
    } else {
        if (2**Math.floor(c) === n) {
            return true
        } else {
            return false
        }
    }
};
