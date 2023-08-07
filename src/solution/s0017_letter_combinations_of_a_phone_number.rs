/**
 * [17] 电话号码的字母组合
 *
 * 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。答案可以按 任意顺序 返回。
 * 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
 * <img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/11/09/200px-telephone-keypad2svg.png" style="width: 200px;" />
 *  
 * 示例 1：
 *
 * 输入：digits = "23"
 * 输出：["ad","ae","af","bd","be","bf","cd","ce","cf"]
 *
 * 示例 2：
 *
 * 输入：digits = ""
 * 输出：[]
 *
 * 示例 3：
 *
 * 输入：digits = "2"
 * 输出：["a","b","c"]
 *
 *  
 * 提示：
 *
 * 	0 <= digits.length <= 4
 * 	digits[i] 是范围 ['2', '9'] 的一个数字。
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/letter-combinations-of-a-phone-number/
// discuss: https://leetcode.cn/problems/letter-combinations-of-a-phone-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        return Self::permutation(&digits);
    }

    fn permutation(digits: &str) -> Vec<String> {
        if digits.len() == 1 {
            let c = digits.as_bytes()[0];
            if c == b'9' {
                return vec![
                    "w".to_string(),
                    "x".to_string(),
                    "y".to_string(),
                    "z".to_string(),
                ];
            } else if c == b'7' {
                return vec![
                    "p".to_string(),
                    "q".to_string(),
                    "r".to_string(),
                    "s".to_string(),
                ];
            } else if c == b'8' {
                return vec!["t".to_string(), "u".to_string(), "v".to_string()];
            }
            let c = (c - b'2') * 3 + b'a';
            return vec![
                String::from_utf8(vec![c]).unwrap(),
                String::from_utf8(vec![c + 1]).unwrap(),
                String::from_utf8(vec![c + 2]).unwrap(),
            ];
        }
        let vec1 = Self::permutation(&digits[0..1]);
        let vec2 = Self::permutation(&digits[1..]);
        let mut res = Vec::new();
        for x in vec1.iter() {
            for y in vec2.iter() {
                res.push(format!("{}{}", x, y));
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a", "b", "c"]
        );
        assert_eq!(
            Solution::letter_combinations("3".to_string()),
            vec!["d", "e", "f"]
        );
        assert_eq!(
            Solution::letter_combinations("9".to_string()),
            vec!["w", "x", "y", "z"]
        );
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        let emptyvec: Vec<String> = vec![];
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            emptyvec
        );
    }
}
