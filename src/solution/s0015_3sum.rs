/**
 * [15] 三数之和
 *
 * 给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请
 * 你返回所有和为 0 且不重复的三元组。
 * 注意：答案中不可以包含重复的三元组。
 *  
 *  
 * 示例 1：
 *
 * 输入：nums = [-1,0,1,2,-1,-4]
 * 输出：[[-1,-1,2],[-1,0,1]]
 * 解释：
 * nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
 * nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
 * nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
 * 不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
 * 注意，输出的顺序和三元组的顺序并不重要。
 *
 * 示例 2：
 *
 * 输入：nums = [0,1,1]
 * 输出：[]
 * 解释：唯一可能的三元组和不为 0 。
 *
 * 示例 3：
 *
 * 输入：nums = [0,0,0]
 * 输出：[[0,0,0]]
 * 解释：唯一可能的三元组和为 0 。
 *
 *  
 * 提示：
 *
 * 	3 <= nums.length <= 3000
 * 	-10^5 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/3sum/
// discuss: https://leetcode.cn/problems/3sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut sorted = nums.clone();
        sorted.sort();
        let mut res = Vec::new();
        let mut i = 0_usize;
        while (i < sorted.len() - 2) {
            let ts = 0 - sorted[i];
            if ts < sorted[i] * 2 {
                break;
            }
            for tr in Self::find_two_sum(&sorted[i + 1..], ts) {
                res.push(vec![sorted[i], tr.0, tr.1]);
            }
            i = Self::find_next_unique(&sorted, i, true);
        }
        res
    }

    #[inline(always)]
    fn find_two_sum(nums: &[i32], sum: i32) -> Vec<(i32, i32)> {
        let (mut i, mut j) = (0, nums.len() - 1);
        let mut res = Vec::new();
        // i < j 即可，太多判断容易错
        while i < j {
            // println!("i: {}, j: {}", i, j);
            if nums[i] + nums[j] > sum {
                j = Self::find_next_unique(nums, j, false);
            } else if nums[i] + nums[j] < sum {
                i = Self::find_next_unique(nums, i, true);
            } else {
                res.push((nums[i], nums[j]));
                i = Self::find_next_unique(nums, i, true);
                j = Self::find_next_unique(nums, j, false);
            }
        }
        res
    }

    #[inline(always)]
    fn find_next_unique(nums: &[i32], idx: usize, forward: bool) -> usize {
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
    use super::*;

    #[test]
    fn test_15() {
        let empty_vec: Vec<Vec<i32>> = vec![];
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1],]
        );
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0],]);
        assert_eq!(
            Solution::three_sum(vec![
                -7, -4, -6, 6, 4, -6, -9, -10, -7, 5, 3, -1, -5, 8, -1, -2, -8, -1, 5, -3, -5, 4,
                2, -5, -4, 4, 7
            ]),
            vec![
                vec![-10, 2, 8],
                vec![-10, 3, 7],
                vec![-10, 4, 6],
                vec![-10, 5, 5],
                vec![-9, 2, 7],
                vec![-9, 3, 6],
                vec![-9, 4, 5],
                vec![-8, 2, 6],
                vec![-8, 3, 5],
                vec![-8, 4, 4],
                vec![-7, -1, 8],
                vec![-7, 2, 5],
                vec![-7, 3, 4],
                vec![-6, -2, 8],
                vec![-6, -1, 7],
                vec![-6, 2, 4],
                vec![-5, -3, 8],
                vec![-5, -2, 7],
                vec![-5, -1, 6],
                vec![-5, 2, 3],
                vec![-4, -4, 8],
                vec![-4, -3, 7],
                vec![-4, -2, 6],
                vec![-4, -1, 5],
                vec![-3, -2, 5],
                vec![-3, -1, 4],
                vec![-2, -1, 3],
                vec![-1, -1, 2]
            ]
        );
        assert_eq!(
            Solution::three_sum(vec![2, 0, -2, -5, -5, -3, 2, -4]),
            vec![vec![-4, 2, 2], vec![-2, 0, 2]]
        );
        assert_eq!(Solution::three_sum(vec![0, 0]), empty_vec);
    }
}
