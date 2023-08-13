/**
 * [28] 找出字符串中第一个匹配项的下标
 *
 * 给你两个字符串 haystack 和 needle ，请你在 haystack 字符串中找出 needle 字符串的第一个匹配项的下标（下标从 0 开始）。如果 needle 不是 haystack 的一部分，则返回  -1 。
 *  
 * <strong class="example">示例 1：
 * 
 * 输入：haystack = "sadbutsad", needle = "sad"
 * 输出：0
 * 解释："sad" 在下标 0 和 6 处匹配。
 * 第一个匹配项的下标是 0 ，所以返回 0 。
 * 
 * <strong class="example">示例 2：
 * 
 * 输入：haystack = "leetcode", needle = "leeto"
 * 输出：-1
 * 解释："leeto" 没有在 "leetcode" 中出现，所以返回 -1 。
 * 
 *  
 * 提示：
 * 
 * 	1 <= haystack.length, needle.length <= 10^4
 * 	haystack 和 needle 仅由小写英文字符组成
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/find-the-index-of-the-first-occurrence-in-a-string/
// discuss: https://leetcode.cn/problems/find-the-index-of-the-first-occurrence-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1
        }
        let a = haystack.as_bytes();
        let b = needle.as_bytes();
        'outer: for i in 0..(haystack.len() - needle.len() + 1) {
            for j in (0..needle.len()) {
                if a[i + j] != b[j] {
                    continue 'outer;
                }
            }
            return i as i32
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_28() {
        assert_eq!(Solution::str_str("haystack".to_string(), "haystack".to_string()), 0);
        assert_eq!(Solution::str_str("haystack".to_string(), "needle".to_string()), -1);
    }
}
