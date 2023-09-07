/**
 * [43] 字符串相乘
 *
 * 给定两个以字符串形式表示的非负整数 num1 和 num2，返回 num1 和 num2 的乘积，它们的乘积也表示为字符串形式。
 * 注意：不能使用任何内置的 BigInteger 库或直接将输入转换为整数。
 *  
 * 示例 1:
 * 
 * 输入: num1 = "2", num2 = "3"
 * 输出: "6"
 * 示例 2:
 * 
 * 输入: num1 = "123", num2 = "456"
 * 输出: "56088"
 *  
 * 提示：
 * 
 * 	1 <= num1.length, num2.length <= 200
 * 	num1 和 num2 只能由数字组成。
 * 	num1 和 num2 都不包含任何前导零，除了数字0本身。
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/multiply-strings/
// discuss: https://leetcode.cn/problems/multiply-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::char;

impl Solution {

    fn assign_digit(res: &mut Vec<u32>, k: usize, v: u32) {
        if k >= res.len() {
            res.push(v);
        } else {
            res[k] += v;
        }
        let mut k = k;
        while res[k] > 10 {
            if k == res.len() - 1 {
                res.push(res[k]/10);
                res[k] %= 10;
                break;
            } else {
                res[k+1] += res[k] / 10;
                res[k] %= 10;
            }
            k += 1;
        }
    }

    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        let mut res = vec![];

        let mut digits1 = vec![];
        for c in num1.chars().rev() {
            digits1.push(c.to_digit(10).unwrap());
        }

        for (i, c) in num2.chars().rev().enumerate() {
            let d2 = c.to_digit(10).unwrap();
            for (j, d1) in digits1.iter().enumerate() {
                Self::assign_digit(&mut res, i + j, d1 * d2);
            }
        }
        let mut s = "".to_string();
        for d in res.iter().rev() {
            s.push(char::from_digit(*d, 10).unwrap());
        }
        s
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43() {
        assert_eq!(Solution::multiply("123".to_string(), "23".to_string()), "2829".to_string());
        assert_eq!(Solution::multiply("982634".to_string(), "2343".to_string()), "2302311462".to_string());
    }
}
