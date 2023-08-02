/**
 * [12] 整数转罗马数字
 *
 * 罗马数字包含以下七种字符： I， V， X， L，C，D 和 M。
 * 
 * 字符          数值
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 * 例如， 罗马数字 2 写做 II ，即为两个并列的 1。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。
 * 通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：
 * 
 * 	I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
 * 	X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。 
 * 	C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
 * 
 * 给你一个整数，将其转为罗马数字。
 *  
 * 示例 1:
 * 
 * 输入: num = 3
 * 输出: "III"
 * 示例 2:
 * 
 * 输入: num = 4
 * 输出: "IV"
 * 示例 3:
 * 
 * 输入: num = 9
 * 输出: "IX"
 * 示例 4:
 * 
 * 输入: num = 58
 * 输出: "LVIII"
 * 解释: L = 50, V = 5, III = 3.
 * 
 * 示例 5:
 * 
 * 输入: num = 1994
 * 输出: "MCMXCIV"
 * 解释: M = 1000, CM = 900, XC = 90, IV = 4.
 *  
 * 提示：
 * 
 * 	1 <= num <= 3999
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/integer-to-roman/
// discuss: https://leetcode.cn/problems/integer-to-roman/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut x = num;
        let mut res = String::new();
        while x > 0 {
            if x >= 1000 {
                x -= 1000;
                res.push('M');
            } else if x >= 900 {
                x -= 900;
                res.push_str("CM");
            } else if x >= 500 {
                x -= 500;
                res.push('D');
            } else if x >= 400 {
                x -= 400;
                res.push_str("CD");
            } else if x >= 100 {
                x -= 100;
                res.push('C');
            } else if x >= 90 {
                x -= 90;
                res.push_str("XC");
            } else if x >= 50 {
                x -= 50;
                res.push('L');
            } else if x >= 40 {
                x -= 40;
                res.push_str("XL");
            } else if x >= 10 {
                x -= 10;
                res.push('X');
            } else if x >= 9 {
                x -= 9;
                res.push_str("IX")
            } else if x >= 5 {
                x -= 5;
                res.push('V');
            } else if x >= 4 {
                x -= 4;
                res.push_str("IV");
            } else {
                x -= 1;
                res.push('I');
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use crate::solution;

    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(Solution::int_to_roman(1), "I");
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
    }
}
