/**
 * [44] 通配符匹配
 *
 * <div class="title__3Vvk">给你一个输入字符串 (s) 和一个字符模式 (p) ，请你实现一个支持 '?' 和 '*' 匹配规则的通配符匹配：</div>
 * 
 * 	<li class="title__3Vvk">'?' 可以匹配任何单个字符。
 * 	<li class="title__3Vvk">'*' 可以匹配任意字符序列（包括空字符序列）。
 * 
 * <div class="original__bRMd">
 * <div>
 * 判定匹配成功的充要条件是：字符模式必须能够 完全匹配 输入字符串（而不是部分匹配）。
 * </div>
 * </div>
 *  
 * <strong class="example">示例 1：
 * 
 * 输入：s = "aa", p = "a"
 * 输出：false
 * 解释："a" 无法匹配 "aa" 整个字符串。
 * 
 * <strong class="example">示例 2：
 * 
 * 输入：s = "aa", p = "*"
 * 输出：true
 * 解释：'*' 可以匹配任意字符串。
 * 
 * <strong class="example">示例 3：
 * 
 * 输入：s = "cb", p = "?a"
 * 输出：false
 * 解释：'?' 可以匹配 'c', 但第二个 'a' 无法匹配 'b'。
 * 
 *  
 * 提示：
 * 
 * 	0 <= s.length, p.length <= 2000
 * 	s 仅由小写英文字母组成
 * 	p 仅由小写英文字母、'?' 或 '*' 组成
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/wildcard-matching/
// discuss: https://leetcode.cn/problems/wildcard-matching/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut dp: Vec<Vec<bool>> = vec![vec![false; p.len()+1]; s.len()+1];
        dp[0][0] = true;
        for j in 1..=p.len() {
            if p[j-1] == b'*' {
                dp[0][j] = true;
            } else {
                break;
            }
        }
        for i in 1..=s.len() {
            for j in 1..=p.len() {
                match p[j-1] {
                    b'?' => {
                        dp[i][j] = dp[i-1][j-1];
                    },
                    b'*' => {
                        dp[i][j] = dp[i-1][j] | dp[i][j-1];
                    },
                    _ => {
                        if s[i-1] == p[j-1] {
                            dp[i][j] = dp[i-1][j-1];
                        }
                    },
                }
            }
        }
        dp[s.len()][p.len()]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_44() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
        assert_eq!(Solution::is_match("hello wold".to_string(), "he*w?ld".to_string()), true);
        assert_eq!(Solution::is_match("cb".to_string(), "?a".to_string()), false);
        assert_eq!(Solution::is_match("".to_string(), "****".to_string()), true);
        assert_eq!(Solution::is_match("acdcb".to_string(), "a*c?b".to_string()), false);
        assert_eq!(Solution::is_match("mississippi".to_string(), "m??*ss*?i*pi".to_string()), false);
        assert_eq!(Solution::is_match("bbbbbbbabbaabbabbbbaaabbabbabaaabbababbbabbbabaaabaab".to_string(), "b*b*ab**ba*b**b***bba".to_string()), false);
    }
}
