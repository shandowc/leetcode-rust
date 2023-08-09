/**
 * [22] 括号生成
 *
 * 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
 *  
 * 示例 1：
 *
 * 输入：n = 3
 * 输出：["((()))","(()())","(())()","()(())","()()()"]
 *
 * 示例 2：
 *
 * 输入：n = 1
 * 输出：["()"]
 *
 *  
 * 提示：
 *
 * 	1 <= n <= 8
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/generate-parentheses/
// discuss: https://leetcode.cn/problems/generate-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::gen(0, 0, n)
    }

    fn gen(l: i32, r: i32, n: i32) -> Vec<String> {
        if l > n || l < r {
            return vec![];
        }
        if l == n && l == r {
            return vec!["".to_string()];
        }
        let mut res = vec![];
        let subs = Self::gen(l + 1, r, n);
        for s in subs {
            res.push(format!("({}", s));
        }
        let subs = Self::gen(l, r + 1, n);
        for s in subs {
            res.push(format!("){}", s));
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()".to_string()]);
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
