/**
 * [45] 跳跃游戏 II
 *
 * 给定一个长度为 n 的 0 索引整数数组 nums。初始位置为 nums[0]。
 * 每个元素 nums[i] 表示从索引 i 向前跳转的最大长度。换句话说，如果你在 nums[i] 处，你可以跳转到任意 nums[i + j] 处:
 * 
 * 	0 <= j <= nums[i] 
 * 	i + j < n
 * 
 * 返回到达 nums[n - 1] 的最小跳跃次数。生成的测试用例可以到达 nums[n - 1]。
 *  
 * 示例 1:
 * 
 * 输入: nums = [2,3,1,1,4]
 * 输出: 2
 * 解释: 跳到最后一个位置的最小跳跃数是 2。
 *      从下标为 0 跳到下标为 1 的位置，跳 1 步，然后跳 3 步到达数组的最后一个位置。
 * 
 * 示例 2:
 * 
 * 输入: nums = [2,3,0,1,4]
 * 输出: 2
 * 
 *  
 * 提示:
 * 
 * 	1 <= nums.length <= 10^4
 * 	0 <= nums[i] <= 1000
 * 	题目保证可以到达 nums[n-1]
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/jump-game-ii/
// discuss: https://leetcode.cn/problems/jump-game-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // 动态规划可以，但是有点慢
        // let mut jp = vec![10000; nums.len()];
        // jp[0] = 0;
        // for i in 0..nums.len() {
        //     for j in 1..=(nums[i] as usize) {
        //         if i + j >= nums.len() {
        //             break;
        //         }
        //         if jp[i+j] > jp[i] + 1 {
        //             jp[i+j] = jp[i] + 1
        //         }
        //     }
        // }
        // jp[nums.len()-1]

        let mut step = 0;
        // 本次step元素段的范围
        let mut end = 0;
        // 本元素段内元素能到达的最远范围，也就是下次step的end
        let mut fartest = 0;
        for i in 0..nums.len() {
            // 本元素段已经能到达末尾，直接返回本step即可
            if end >= nums.len() - 1 {
                return step
            }
            let forward = i + (nums[i] as usize);
            fartest = if fartest < forward { forward } else { fartest };
            if i == end {
                end = fartest;
                step += 1;
            }
        }
        step
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_45() {
        assert_eq!(Solution::jump(vec![2,3,1,4]), 2);
        assert_eq!(Solution::jump(vec![2,3,0,1,4]), 2);
        assert_eq!(Solution::jump(vec![2,0,2,0,1]), 2);
    }
}
