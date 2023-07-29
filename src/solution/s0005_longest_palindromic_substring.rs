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
        let mut i = i as i32 - 1;
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

    // 朴素算法
    fn longest_palindrome_native(s: String) -> String {
        let bytes = s.as_bytes();
        let (mut start, mut end, mut max) = (0usize, 0usize, 1usize);
        for i in (0..bytes.len()) {
            let max1 = Solution::find_palindrome_from_center_odd(bytes, i);
            let max2 = Solution::find_palindrome_from_center_even(bytes, i);
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
        s[start..=end].to_owned()
    }

    // 马拉车算法
    fn longest_palindrome_manacher(s: String) -> String {
        let mut new_s = vec![b'd'; s.len() * 2 + 1];
        // 1. 通过插入同一个字符，保证新字符串的奇数子字符串只要是回文串，则里面包含的原来字符的串也是回文串
        for (i, ele) in new_s.iter_mut().enumerate() {
            *ele = if i % 2 == 0 {
                b'#'
            } else {
                s.as_bytes()[i / 2]
            }
        }

        let mut cache = vec![0; new_s.len()];
        let mut mid = 0usize;
        let mut rightmost = 0usize;

        let mut max_palindrome_len = 1usize;
        let mut max_palindrome_l = 0usize;
        let mut max_palindrome_r = 0usize;

        let new_len = new_s.len() as i32;
        for i in (0..cache.len()) {
            /*
            2. 马拉车算法核心，当前位置如果在某个之前已经遍历过的回文串a中间，则以下应相等
                1) 当前位置回文串b在a中的子串长度
                2) 对称位置回文串c在a中的子串长度
             */
            let mut r = if i < rightmost {
                std::cmp::min(cache[mid - (i - mid)] / 2 + i, rightmost)
            } else {
                i
            } as i32;

            let mut l = 2 * i as i32 - r;
            while l - 1 >= 0 && r + 1 < new_len && new_s[l as usize - 1] == new_s[r as usize + 1] {
                r += 1;
                l -= 1;
            }
            let l = l as usize;
            let r = r as usize;
            if r > rightmost {
                rightmost = r as usize;
                mid = i;
            }
            let cur_pal_len = r - l + 1;
            cache[i] = cur_pal_len;
            if cur_pal_len > max_palindrome_len {
                max_palindrome_len = cur_pal_len;
                max_palindrome_l = l;
                max_palindrome_r = r;
            }
        }
        s[(max_palindrome_l / 2)..(max_palindrome_r / 2)].into()
    }

    pub fn longest_palindrome(s: String) -> String {
        // Self::longest_palindrome_native(s)
        Self::longest_palindrome_manacher(s)
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

        let longest = Solution::longest_palindrome("bb".to_string());
        assert_eq!(longest, "bb");
    }
}
