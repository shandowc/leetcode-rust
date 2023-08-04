/**
 * [14] 最长公共前缀
 *
 * 编写一个函数来查找字符串数组中的最长公共前缀。
 * 如果不存在公共前缀，返回空字符串 ""。
 *  
 * 示例 1：
 *
 * 输入：strs = ["flower","flow","flight"]
 * 输出："fl"
 *
 * 示例 2：
 *
 * 输入：strs = ["dog","racecar","car"]
 * 输出：""
 * 解释：输入不存在公共前缀。
 *  
 * 提示：
 *
 * 	1 <= strs.length <= 200
 * 	0 <= strs[i].length <= 200
 * 	strs[i] 仅由小写英文字母组成
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/longest-common-prefix/
// discuss: https://leetcode.cn/problems/longest-common-prefix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut its: Vec<Box<dyn Iterator<Item = &u8>>> = strs[1..]
            .iter()
            .map(|s| Box::new(s.as_bytes().iter()) as Box<dyn Iterator<Item = &u8>>)
            .collect();
        let mut end = 0;
        for c in strs[0].bytes() {
            for it in &mut its {
                let r = it.next();
                if r.is_none() || *r.unwrap() != c {
                    return String::from(&strs[0].as_str()[..end]);
                }
            }
            end += 1;
        }
        strs[0].to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        Solution::longest_common_prefix(vec!["1".to_string()]);
        assert_eq!(
            Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec!["pig".to_string(), "sheep".to_string(), "flight".to_string()]),
            ""
        );
    }
}
