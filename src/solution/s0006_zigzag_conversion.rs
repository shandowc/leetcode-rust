use std::rc::Rc;

/**
 * [6] N 字形变换
 *
 * 将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。
 * 比如输入字符串为 "PAYPALISHIRING" 行数为 3 时，排列如下：
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 * 之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："PAHNAPLSIIGYIR"。
 * 请你实现这个将字符串进行指定行数变换的函数：
 *
 * string convert(string s, int numRows);
 *
 * 示例 1：
 *
 * 输入：s = "PAYPALISHIRING", numRows = 3
 * 输出："PAHNAPLSIIGYIR"
 * 示例 2：
 *
 * 输入：s = "PAYPALISHIRING", numRows = 4
 * 输出："PINALSIGYAHRPI"
 * 解释：
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 * 示例 3：
 *
 * 输入：s = "A", numRows = 1
 * 输出："A"
 *
 *
 * 提示：
 *
 * 	1 <= s.length <= 1000
 * 	s 由英文字母（小写和大写）、',' 和 '.' 组成
 * 	1 <= numRows <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/zigzag-conversion/
// discuss: https://leetcode.cn/problems/zigzag-conversion/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here


impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s.into();
        }
        let bytes = s.as_bytes();
        let mut row: Vec<Vec<u8>> = vec![Vec::with_capacity(s.len()/(num_rows as usize - 1)); num_rows as usize];

        let nums_grp = (num_rows - 1) * 2;
        for i in (0..s.len()) {
            let rem = (i as i32) % nums_grp;
            let row_no = if rem > num_rows - 1 {
                (num_rows - 1) * 2 - rem
            } else {
                rem
            };
            row[row_no as usize].push(bytes[i]);
        }
        let mut res = Vec::new();
        for i in (0..num_rows as usize) {
            res.extend(row[i].iter());
        }
        std::str::from_utf8(&res).unwrap().to_owned()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(
            Solution::convert("ABC".to_string(), 1),
            "ABC"
        );
        assert_eq!(
            Solution::convert("ACB".to_string(), 1),
            "ACB"
        );
    }
}
