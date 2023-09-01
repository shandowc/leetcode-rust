use std::vec;

/**
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 *
 * 给你一个按照非递减顺序排列的整数数组 nums，和一个目标值 target。请你找出给定目标值在数组中的开始位置和结束位置。
 * 如果数组中不存在目标值 target，返回 [-1, -1]。
 * 你必须设计并实现时间复杂度为 O(log n) 的算法解决此问题。
 *  
 * 示例 1：
 *
 * 输入：nums = [5,7,7,8,8,10], target = 8
 * 输出：[3,4]
 * 示例 2：
 *
 * 输入：nums = [5,7,7,8,8,10], target = 6
 * 输出：[-1,-1]
 * 示例 3：
 *
 * 输入：nums = [], target = 0
 * 输出：[-1,-1]
 *  
 * 提示：
 *
 * 	0 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 	nums 是一个非递减数组
 * 	-10^9 <= target <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/
// discuss: https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }

        let mut left_res = -1;
        let mut left = 0;
        let mut right = nums.len() - 1;
        while (left <= right) {
            let mid = (left + right) / 2;
            if target == nums[mid] {
                if mid == 0 || nums[mid - 1] != target {
                    left_res = mid as i32;
                    break;
                }
                right = mid - 1;
            } else if target > nums[mid] {
                left = mid + 1;
            } else if mid == 0 {
                return vec![-1, -1];
            } else {
                right = mid - 1;
            }
        }
        if left_res == -1 {
            return vec![-1, -1];
        }

        let mut right_res = left_res;
        let mut left = left_res as usize;
        let mut right = nums.len() - 1;
        while (left <= right) {
            let mid = (left + right) / 2;
            if target == nums[mid] {
                if mid == nums.len() - 1 || nums[mid + 1] != target {
                    right_res = mid as i32;
                    break;
                }
                left = mid + 1;
            } else if mid == 0 {
                return vec![-1, -1];
            } else {
                right = mid - 1;
            }
            // 不可能有小于的情况
        }
        return vec![left_res, right_res];
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(
            Solution::search_range(vec![], 0),
            vec![-1, -1]
        );
        assert_eq!(
            Solution::search_range(vec![1], 0),
            vec![-1, -1]
        );
    }
}
