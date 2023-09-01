use std::thread::current;

/**
 * [32] 最长有效括号
 *
 * 给你一个只包含 '(' 和 ')' 的字符串，找出最长有效（格式正确且连续）括号子串的长度。
 *
 * <div class="original__bRMd">
 * <div>
 * 示例 1：
 *
 * 输入：s = "(()"
 * 输出：2
 * 解释：最长有效括号子串是 "()"
 *
 * 示例 2：
 *
 * 输入：s = ")()())"
 * 输出：4
 * 解释：最长有效括号子串是 "()()"
 *
 * 示例 3：
 *
 * 输入：s = ""
 * 输出：0
 *
 *
 * 提示：
 *
 * 	0 <= s.length <= 3 * 10^4
 * 	s[i] 为 '(' 或 ')'
 * </div>
 * </div>
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/longest-valid-parentheses/
// discuss: https://leetcode.cn/problems/longest-valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut parenthesses = vec![];
        let mut max_num = 0;
        let mut con_start = -1i32;
        for (i, b) in s.as_bytes().iter().enumerate() {
            if *b == b'(' {
                parenthesses.push((i, b));
                continue;
            }
            if parenthesses.len() == 0 {
                parenthesses.clear();
                con_start = i as i32;
                continue;
            }
            parenthesses.pop();

            let cur_num = if parenthesses.len() == 0 {
                (i as i32 - con_start) as usize
            } else {
                i - parenthesses[parenthesses.len()-1].0
            };
            if cur_num > max_num {
                max_num = cur_num;
            }
        }
        max_num as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_32() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("(())(".to_string()), 4);
    }
}
