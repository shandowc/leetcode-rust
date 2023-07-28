/**
 * [5] 最长回文子串
 *
 * 给你一个字符串 s，找到 s 中最长的回文子串。
 * 如果字符串的反序与原始字符串相同，则该字符串称为回文字符串。
 *  
 * 示例 1：
 *
 * 输入：s = "babad"
 * 输出："bab"
 * 解释："aba" 同样是符合题意的答案。
 *
 * 示例 2：
 *
 * 输入：s = "cbbd"
 * 输出："bb"
 *
 *  
 * 提示：
 *
 * 	1 <= s.length <= 1000
 * 	s 仅由数字和英文字母组成
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/longest-palindromic-substring/
// discuss: https://leetcode.cn/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn find_palindrome_from_center_odd(s: &[u8], i: usize) -> (usize) {
        let mut max = 1usize;
        let mut i = (i - 1) as i32;
        let mut j = i + 2;
        while i >= 0 && j < s.len() as i32 {
            if s[i as usize] != s[j as usize] {
                break;
            }
            i -= 1;
            j += 1;
            max += 2;
        }
        max
    }

    fn find_palindrome_from_center_even(s: &[u8], i: usize) -> usize {
        let mut max = 0usize;
        let mut i = i as i32;
        let mut j = i + 1;

        while i >= 0 && j < s.len() as i32 {
            if s[i as usize] != s[j as usize] {
                break;
            }
            i -= 1;
            j += 1;
            max += 2;
        }
        max
    }

    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let (mut start, mut end, mut max) = (0usize, 0usize, 1usize);
        for i in (1..s.len()) {
            let max1 = Solution::find_palindrome_from_center_odd(s, i);
            let max2 = Solution::find_palindrome_from_center_even(s, i);
            if max1 > max2 && max1 > max {
                start = i - max1 / 2;
                end = i + max1 / 2;
                max = max1;
            } else if max2 > max1 && max2 > max {
                start = i - (max2 - 1) / 2;
                end = i + max2 / 2;
                max = max2;
            }
        }
        std::str::from_utf8(&s[start..=end]).unwrap().to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        let longest = Solution::longest_palindrome("babad".to_string());
        assert!(longest == "bab" || longest == "aba");

        let longest = Solution::longest_palindrome("sbbd".to_string());
        assert_eq!(longest, "bb");

        let longest = Solution::longest_palindrome("banana".to_string());
        assert_eq!(longest, "anana");
    }
}
