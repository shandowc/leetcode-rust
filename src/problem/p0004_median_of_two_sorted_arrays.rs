/**
 * [4] 寻找两个正序数组的中位数
 *
 * 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
 * 算法的时间复杂度应该为 O(log (m+n)) 。
 *  
 * 示例 1：
 * 
 * 输入：nums1 = [1,3], nums2 = [2]
 * 输出：2.00000
 * 解释：合并数组 = [1,2,3] ，中位数 2
 * 
 * 示例 2：
 * 
 * 输入：nums1 = [1,2], nums2 = [3,4]
 * 输出：2.50000
 * 解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5
 * 
 *  
 *  
 * 提示：
 * 
 * 	nums1.length == m
 * 	nums2.length == n
 * 	0 <= m <= 1000
 * 	0 <= n <= 1000
 * 	1 <= m + n <= 2000
 * 	-10^6 <= nums1[i], nums2[i] <= 10^6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/median-of-two-sorted-arrays/
// discuss: https://leetcode.cn/problems/median-of-two-sorted-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        0f64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        // assert_eq!(
        //     Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        //     2f64
        // );
        // assert_eq!(
        //     Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        //     2.5f64
        // );
        // assert_eq!(
        //     Solution::find_median_sorted_arrays(vec![1, 2, 7], vec![3, 4]),
        //     3f64
        // );
        // assert_eq!(
        //     Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4, 5, 6, 7, 8]),
        //     4.5f64
        // );
        // assert_eq!(
        //     Solution::find_median_sorted_arrays(vec![], vec![3, 4, 5, 6, 7, 8]),
        //     5.5f64
        // );
        // assert_eq!(
        //     Solution::find_median_sorted_arrays(vec![3, 4, 5, 6, 7, 8], vec![]),
        //     5.5f64
        // );
    }
}
