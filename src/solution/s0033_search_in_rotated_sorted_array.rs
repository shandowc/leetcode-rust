/**
 * [33] 搜索旋转排序数组
 *
 * 整数数组 nums 按升序排列，数组中的值 互不相同 。
 * 在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行了 旋转，使数组变为 [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]（下标 从 0 开始 计数）。例如， [0,1,2,4,5,6,7] 在下标 3 处经旋转后可能变为 [4,5,6,7,0,1,2] 。
 * 给你 旋转后 的数组 nums 和一个整数 target ，如果 nums 中存在这个目标值 target ，则返回它的下标，否则返回 -1 。
 * 你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。
 *  
 * 示例 1：
 * 
 * 输入：nums = [4,5,6,7,0,1,2], target = 0
 * 输出：4
 * 
 * 示例 2：
 * 
 * 输入：nums = [4,5,6,7,0,1,2], target = 3
 * 输出：-1
 * 示例 3：
 * 
 * 输入：nums = [1], target = 0
 * 输出：-1
 * 
 *  
 * 提示：
 * 
 * 	1 <= nums.length <= 5000
 * 	-10^4 <= nums[i] <= 10^4
 * 	nums 中的每个值都 独一无二
 * 	题目数据保证 nums 在预先未知的某个下标上进行了旋转
 * 	-10^4 <= target <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/search-in-rotated-sorted-array/
// discuss: https://leetcode.cn/problems/search-in-rotated-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while (left <= right) {
            let mid = (left + right) / 2;
            if nums[mid] == target {
                return mid as i32;
            }

            // 正常二分查找
            if nums[left] < nums[right] {
                if target > nums[mid] {
                    left = mid + 1;
                } else if mid == 0 {
                    return -1;
                } else {
                    right = mid - 1;
                }
                continue;
            }

            // 左半部分是正常二分，且target在左半部分范围内
            if nums[left] < nums[mid] {
                if target >= nums[left] && target < nums[mid] {
                    right = mid - 1;
                    continue;
                }
            }

            // 右半部分是正常二分，且target在右半部分范围内
            if nums[mid] < nums[right] {
                if target > nums[mid] && target <= nums[right] {
                    left = mid + 1;
                    continue;
                }
            }

            // 不在正常的那边则在不正常的那半边，无需额外条件
            if nums[right] < nums[mid] {
                left = mid + 1
            } else if mid == 0 {
                return -1;
            } else {
                right = mid - 1;
            }
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_33() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 0), 4);
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
        assert_eq!(Solution::search(vec![1, 3], 0), -1);
        assert_eq!(Solution::search(vec![3, 1], 1), 1);
    }
}
