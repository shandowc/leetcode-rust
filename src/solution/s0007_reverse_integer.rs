/**
 * [7] 整数反转
 *
 * 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。
 * 如果反转后整数超过 32 位的有符号整数的范围 [−2^31,  2^31 − 1] ，就返回 0。
 * 假设环境不允许存储 64 位整数（有符号或无符号）。
 *
 * 示例 1：
 *
 * 输入：x = 123
 * 输出：321
 *
 * 示例 2：
 *
 * 输入：x = -123
 * 输出：-321
 *
 * 示例 3：
 *
 * 输入：x = 120
 * 输出：21
 *
 * 示例 4：
 *
 * 输入：x = 0
 * 输出：0
 *
 *
 * 提示：
 *
 * 	-2^31 <= x <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/reverse-integer/
// discuss: https://leetcode.cn/problems/reverse-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == std::i32::MIN {
            return 0;
        }
        let mut y = x.abs();
        let mut res: i32 = 0;
        while y > 0 {
            if res.checked_mul(10).is_none() {
                return 0;
            }
            res = res * 10;
            if res.checked_add(y % 10).is_none() {
                return 0;
            }
            res = res + y % 10;
            y = y / 10;
        }
        if x < 0 {
            -res
        } else {
            res
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(std::i32::MAX), 0);
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
    }
}
