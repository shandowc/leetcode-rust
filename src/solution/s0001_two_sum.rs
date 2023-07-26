/**
 * [1] 两数之和
 *
 * 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
 * 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
 * 你可以按任意顺序返回答案。
 *  
 * <strong class="example">示例 1：
 *
 * 输入：nums = [2,7,11,15], target = 9
 * 输出：[0,1]
 * 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
 *
 * <strong class="example">示例 2：
 *
 * 输入：nums = [3,2,4], target = 6
 * 输出：[1,2]
 *
 * <strong class="example">示例 3：
 *
 * 输入：nums = [3,3], target = 6
 * 输出：[0,1]
 *
 *  
 * 提示：
 *
 * 	2 <= nums.length <= 10^4
 * 	-10^9 <= nums[i] <= 10^9
 * 	-10^9 <= target <= 10^9
 * 	只会存在一个有效答案
 *
 *  
 * 进阶：你可以想出一个时间复杂度小于 O(n^2) 的算法吗？
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/two-sum/
// discuss: https://leetcode.cn/problems/two-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::{collections::HashMap, ops::Deref};

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            let wanted = target - x;
            if m.contains_key(&wanted) {
                return vec![*(m.get(&wanted).unwrap()), i as i32];
            }
            m.insert(*x, i as i32);
        }
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
    }
}
