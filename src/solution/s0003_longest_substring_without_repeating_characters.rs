/**
 * [3] 无重复字符的最长子串
 *
 * 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
 *  
 * 示例 1:
 *
 * 输入: s = "abcabcbb"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
 *
 * 示例 2:
 *
 * 输入: s = "bbbbb"
 * 输出: 1
 * 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
 *
 * 示例 3:
 *
 * 输入: s = "pwwkew"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
 *      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
 *
 *  
 * 提示：
 *
 * 	0 <= s.length <= 5 * 10^4
 * 	s 由英文字母、数字、符号和空格组成
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.cn/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let s: Vec<char> = s.chars().collect();
        let mut charmap = HashMap::new();
        let mut longest_len = 1;

        let mut p = 0;
        let mut i = 0;

        charmap.insert(s[0], 1);

        for i in (1..s.len()) {
            let c = s[i];
            if charmap.contains_key(&c) {
                while s[p] != c {
                    charmap.remove(&s[p]);
                    p += 1;
                }
                p += 1;
            } else {
                charmap.insert(c, 1);
            }
            let cur_long = i - p + 1;
            if cur_long > longest_len {
                longest_len = cur_long;
            }
        }
        return longest_len as i32;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
