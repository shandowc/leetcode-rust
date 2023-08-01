/**
 * [11] 盛最多水的容器
 *
 * 给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i]) 。
 * 找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
 * 返回容器可以储存的最大水量。
 * 说明：你不能倾斜容器。
 *  
 * 示例 1：
 * <img alt="" src="https://aliyun-lc-upload.oss-cn-hangzhou.aliyuncs.com/aliyun-lc-upload/uploads/2018/07/25/question_11.jpg" />
 * 
 * 输入：[1,8,6,2,5,4,8,3,7]
 * 输出：49 
 * 解释：图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
 * 示例 2：
 * 
 * 输入：height = [1,1]
 * 输出：1
 * 
 *  
 * 提示：
 * 
 * 	n == height.length
 * 	2 <= n <= 10^5
 * 	0 <= height[i] <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/container-with-most-water/
// discuss: https://leetcode.cn/problems/container-with-most-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, height.len()-1);
        let (mut maxl, mut maxr, mut max_area) = (0usize, 0usize, 0usize);
        while i < j {
            let area = (j - i) * std::cmp::min(height[j], height[i]) as usize;
            if area > max_area {
                max_area = area;
                maxl = i;
                maxr = j;
            }
            // 不断把短板替换掉
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        max_area as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
