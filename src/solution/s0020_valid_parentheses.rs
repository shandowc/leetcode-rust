/**
 * [20] 有效的括号
 *
 * 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
 * 有效字符串需满足：
 * <ol>
 * 	左括号必须用相同类型的右括号闭合。
 * 	左括号必须以正确的顺序闭合。
 * 	每个右括号都有一个对应的相同类型的左括号。
 * </ol>
 *  
 * 示例 1：
 * 
 * 输入：s = "()"
 * 输出：true
 * 
 * 示例 2：
 * 
 * 输入：s = "()[]{}"
 * 输出：true
 * 
 * 示例 3：
 * 
 * 输入：s = "(]"
 * 输出：false
 * 
 *  
 * 提示：
 * 
 * 	1 <= s.length <= 10^4
 * 	s 仅由括号 '()[]{}' 组成
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/valid-parentheses/
// discuss: https://leetcode.cn/problems/valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut brackets = vec![];
        for c in s.as_bytes() {
            match c {
                b'(' | b'[' | b'{' => {
                    brackets.push(*c);
                },
                b')' | b']' | b'}' => {
                    if brackets.last().is_none() {
                        return false;
                    }
                    let t = brackets.pop().unwrap();
                    if *c == b')' && t != b'(' {
                        return false;
                    } else if *c == b']' && t != b'[' {
                        return false;
                    } else if *c == b'}' && t != b'{' {
                        return false;
                    }
                    
                },
                _ => {},
            }
        }
        brackets.is_empty()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        assert_eq!(Solution::is_valid("[".to_string()), false);
        assert_eq!(Solution::is_valid("]]]".to_string()), false);
        assert_eq!(Solution::is_valid("[sdfs]".to_string()), true);
        assert_eq!(Solution::is_valid("[]".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("[)".to_string()), false);
    }
}
