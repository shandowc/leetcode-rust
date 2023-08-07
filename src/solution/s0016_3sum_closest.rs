/**
 * [16] 最接近的三数之和
 *
 * 给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。
 * 返回这三个数的和。
 * 假定每组输入只存在恰好一个解。
 *  
 * 示例 1：
 *
 * 输入：nums = [-1,2,1,-4], target = 1
 * 输出：2
 * 解释：与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。
 *
 * 示例 2：
 *
 * 输入：nums = [0,0,0], target = 1
 * 输出：0
 *
 *  
 * 提示：
 *
 * 	3 <= nums.length <= 1000
 * 	-1000 <= nums[i] <= 1000
 * 	-10^4 <= target <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/3sum-closest/
// discuss: https://leetcode.cn/problems/3sum-closest/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let (mut i, mut j, mut k) = (0, 0, 0);
        let mut closest = nums[0] + nums[1] + nums[2];
        while i < nums.len() - 2 {
            j = i + 1;
            k = nums.len() - 1;
            while j < k {
                let curr = nums[i] + nums[j] + nums[k];
                if curr == target {
                    return target;
                }
                if (curr - target).abs() < (closest - target).abs() {
                    closest = curr;
                }
                if curr < target {
                    j = Self::find_next_unique(&nums, j, true);
                } else {
                    k = Self::find_next_unique(&nums, k, false);
                }
            }
            i = Self::find_next_unique(&nums, i, true);
        }
        closest
    }

    #[inline(always)]
    fn find_next_unique(nums: &[i32], idx: usize, forward: bool) -> usize {
        let mut i = idx;
        let curr = nums[idx];
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
    use super::*;

    #[test]
    fn test_16() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
        assert_eq!(Solution::three_sum_closest(vec![-1, -2, -3, -4, 1, 1, 1, 1], 0), 0);
    }
}
