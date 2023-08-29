/**
 * [29] 两数相除
 *
 * 给你两个整数，被除数 dividend 和除数 divisor。将两数相除，要求 不使用 乘法、除法和取余运算。
 * 整数除法应该向零截断，也就是截去（truncate）其小数部分。例如，8.345 将被截断为 8 ，-2.7335 将被截断至 -2 。
 * 返回被除数 dividend 除以除数 divisor 得到的 商 。
 * 注意：假设我们的环境只能存储 32 位 有符号整数，其数值范围是 [−2^31,  2^31 − 1] 。本题中，如果商 严格大于 2^31 − 1 ，则返回 2^31 − 1 ；如果商 严格小于 -2^31 ，则返回 -2^31^ 。
 *  
 * 示例 1:
 * 
 * 输入: dividend = 10, divisor = 3
 * 输出: 3
 * 解释: 10/3 = 3.33333.. ，向零截断后得到 3 。
 * 示例 2:
 * 
 * 输入: dividend = 7, divisor = -3
 * 输出: -2
 * 解释: 7/-3 = -2.33333.. ，向零截断后得到 -2 。
 *  
 * 提示：
 * 
 * 	-2^31 <= dividend, divisor <= 2^31 - 1
 * 	divisor != 0
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/divide-two-integers/
// discuss: https://leetcode.cn/problems/divide-two-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn rectify(i: i32) -> u32 {
        if i == std::i32::MIN {
            return i as u32
        } else if i < 0 {
            return -i as u32
        } else {
            return i as u32
        }
    }

    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == 1 {
            return i32::MIN
        }

        let mut a = Self::rectify(dividend);
        let b = Self::rectify(divisor);

        let mut m = 0;
        let mut res = 0;
        let magic = 0x80000000u32;
        for _ in 0..32{
            res = res << 1;
            m = m << 1;
            if a & magic > 0 {
                m += 1;
            }
            a <<= 1;
            if m >= b {
                res += 1;
                m -= b;
            }
        }
        if (res == i32::MIN) {
            return i32::MAX
        } else if (dividend < 0 && divisor < 0) || (dividend > 0 && divisor > 0) {
            return res;
        } else {
            return -res;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_29() {
        assert_eq!(Solution::divide(9, 2), 4);
        assert_eq!(Solution::divide(-9, 2), -4);
        assert_eq!(Solution::divide(i32::MIN, i32::MIN), 1);
        assert_eq!(Solution::divide(i32::MAX, i32::MAX), 1);
        assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
        assert_eq!(Solution::divide(i32::MIN, 1), i32::MIN);
        assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(-1, 7), 0);
    }
}
