/**
 * [35] 搜索插入位置
 *
 * 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
 * 请必须使用时间复杂度为 O(log n) 的算法。
 *  
 * 示例 1:
 * 
 * 输入: nums = [1,3,5,6], target = 5
 * 输出: 2
 * 
 * 示例 2:
 * 
 * 输入: nums = [1,3,5,6], target = 2
 * 输出: 1
 * 
 * 示例 3:
 * 
 * 输入: nums = [1,3,5,6], target = 7
 * 输出: 4
 * 
 *  
 * 提示:
 * 
 * 	1 <= nums.length <= 10^4
 * 	-10^4 <= nums[i] <= 10^4
 * 	nums 为 无重复元素 的 升序 排列数组
 * 	-10^4 <= target <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/search-insert-position/
// discuss: https://leetcode.cn/problems/search-insert-position/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while (left <= right) {
            let mid = (left + right) / 2;
            if target == nums[mid] {
                return mid as i32;
            }
            if target < nums[left] {
                return left as i32;
            }
            if target > nums[right] {
                return (right + 1) as i32;
            }
            if target > nums[mid] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_35() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
