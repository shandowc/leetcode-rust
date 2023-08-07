/**
 * [18] 四数之和
 *
 * 给你一个由 n 个整数组成的数组 nums ，和一个目标值 target 。请你找出并返回满足下述全部条件且不重复的四元组 [nums[a], nums[b], nums[c], nums[d]] （若两个四元组元素一一对应，则认为两个四元组重复）：
 *
 * 	0 <= a, b, c, d < n
 * 	a、b、c 和 d 互不相同
 * 	nums[a] + nums[b] + nums[c] + nums[d] == target
 *
 * 你可以按 任意顺序 返回答案 。
 *  
 * 示例 1：
 *
 * 输入：nums = [1,0,-1,0,-2,2], target = 0
 * 输出：[[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
 *
 * 示例 2：
 *
 * 输入：nums = [2,2,2,2,2], target = 8
 * 输出：[[2,2,2,2]]
 *
 *  
 * 提示：
 *
 * 	1 <= nums.length <= 200
 * 	-10^9 <= nums[i] <= 10^9
 * 	-10^9 <= target <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/4sum/
// discuss: https://leetcode.cn/problems/4sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort();

        let mut res: Vec<Vec<i32>> = vec![];

        let target = target as i64;
        let mut i = 0;
        while i < nums.len() - 3 {
            let mut j = i + 1;
            while j < nums.len() - 2 {
                let mut k = j + 1;
                let mut l = nums.len() - 1;
                while k < l {
                    let curr: i64 = nums[i] as i64 + nums[j] as i64 + nums[k] as i64 + nums[l] as i64;
                    if curr == target {
                        res.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                        k = Self::find_next_uniq(&nums, k, true);
                        l = Self::find_next_uniq(&nums, l, false);
                    } else if curr > target {
                        l = Self::find_next_uniq(&nums, l, false);
                    } else {
                        k = Self::find_next_uniq(&nums, k, true);
                    }
                }
                j = Self::find_next_uniq(&nums, j, true);
            }
            i = Self::find_next_uniq(&nums, i, true);
        }

        res
    }

    #[inline(always)]
    fn find_next_uniq(nums: &[i32], idx: usize, forward: bool) -> usize {
        let curr = nums[idx];
        let mut i = idx;
        if forward {
            while i < nums.len() && nums[i] == curr {
                i += 1;
            }
        } else {
            while i > 0 && nums[i] == curr {
                i -= 1;
            }
        }
        i
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_18() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );

        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![vec![2, 2, 2, 2]]
        );

        let emptyvec: Vec<Vec<i32>> = vec![];
        assert_eq!(
            Solution::four_sum(vec![1000000000, 1000000000, 1000000000, 1000000000], -294967296),
            emptyvec
        );
    }
}
