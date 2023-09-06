/**
 * [41] 缺失的第一个正数
 *
 * 给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。
 * 请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。
 *  
 * 示例 1：
 * 
 * 输入：nums = [1,2,0]
 * 输出：3
 * 
 * 示例 2：
 * 
 * 输入：nums = [3,4,-1,1]
 * 输出：2
 * 
 * 示例 3：
 * 
 * 输入：nums = [7,8,9,11,12]
 * 输出：1
 * 
 *  
 * 提示：
 * 
 * 	1 <= nums.length <= 5 * 10^5
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/first-missing-positive/
// discuss: https://leetcode.cn/problems/first-missing-positive/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums: Vec<i32> = nums;
        for i in 0..nums.len() {
            if nums[i] <= 0 || nums[i] >= nums.len() as i32 {
                continue;
            }
            if (i+1) as i32 != nums[i] {
                let j = (nums[i] - 1) as usize;
                nums.swap(i, j);
            }
        }
        for i in 0..nums.len() {
            if (i+1) as i32 != nums[i] {
                return (i+1) as i32;
            }
        }
        nums.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_41() {
        assert_eq!(
            Solution::first_missing_positive(vec![8,2,3,1,8,1,2,8]),4
        )
    }
}
