/**
 * [31] 下一个排列
 *
 * 整数数组的一个 排列  就是将其所有成员以序列或线性顺序排列。
 * 
 * 	例如，arr = [1,2,3] ，以下这些都可以视作 arr 的排列：[1,2,3]、[1,3,2]、[3,1,2]、[2,3,1] 。
 * 
 * 整数数组的 下一个排列 是指其整数的下一个字典序更大的排列。更正式地，如果数组的所有排列根据其字典顺序从小到大排列在一个容器中，那么数组的 下一个排列 就是在这个有序容器中排在它后面的那个排列。如果不存在下一个更大的排列，那么这个数组必须重排为字典序最小的排列（即，其元素按升序排列）。
 * 
 * 	例如，arr = [1,2,3] 的下一个排列是 [1,3,2] 。
 * 	类似地，arr = [2,3,1] 的下一个排列是 [3,1,2] 。
 * 	而 arr = [3,2,1] 的下一个排列是 [1,2,3] ，因为 [3,2,1] 不存在一个字典序更大的排列。
 * 
 * 给你一个整数数组 nums ，找出 nums 的下一个排列。
 * 必须<a href="https://baike.baidu.com/item/%E5%8E%9F%E5%9C%B0%E7%AE%97%E6%B3%95" target="_blank"> 原地 </a>修改，只允许使用额外常数空间。
 *  
 * 示例 1：
 * 
 * 输入：nums = [1,2,3]
 * 输出：[1,3,2]
 * 
 * 示例 2：
 * 
 * 输入：nums = [3,2,1]
 * 输出：[1,2,3]
 * 
 * 示例 3：
 * 
 * 输入：nums = [1,1,5]
 * 输出：[1,5,1]
 * 
 *  
 * 提示：
 * 
 * 	1 <= nums.length <= 100
 * 	0 <= nums[i] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/next-permutation/
// discuss: https://leetcode.cn/problems/next-permutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let num_len = nums.len();
        let mut max_idx = num_len - 1;
        let mut max = nums[max_idx];
        for i in (0..num_len-1).rev() {
            if nums[i] >= max {
                max_idx = i;
                max = nums[i];
            } else {
                nums.swap(i, max_idx);
                nums[i..num_len-1].sort();
                return;
            }
        }
        nums.sort();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);

        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);

        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);

        let mut nums = vec![8, 8, 4, 4];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![4, 4, 8, 8]);
    }
}
