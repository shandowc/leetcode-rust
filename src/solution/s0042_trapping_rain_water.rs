/**
 * [42] 接雨水
 *
 * 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
 *  
 * 示例 1：
 * <img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/22/rainwatertrap.png" style="height: 161px; width: 412px;" />
 * 
 * 输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
 * 输出：6
 * 解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。 
 * 
 * 示例 2：
 * 
 * 输入：height = [4,2,0,3,2,5]
 * 输出：9
 * 
 *  
 * 提示：
 * 
 * 	n == height.length
 * 	1 <= n <= 2 * 10^4
 * 	0 <= height[i] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/trapping-rain-water/
// discuss: https://leetcode.cn/problems/trapping-rain-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack: Vec<(usize, i32)> = vec![];
        let mut res = 0;
        let mut bsum = 0;
        for (i, v) in height.iter().enumerate() {
            if stack.len() > 0 {
                if *v >= stack[0].1 {
                    let (j, h) = stack[0];
                    res += (i - j) as i32 * h - bsum;
                    stack.clear();
                    bsum = 0;
                } else {
                    while stack[stack.len()-1].1 < *v {
                        stack.pop();
                    }
                }
            }
            bsum += v;
            stack.push((i, *v));
        }
        if stack.len() > 1 {
            let mut sidx = stack[0].0;
            for (eidx, h) in &stack[1..] {
                bsum = 0;
                for i in (sidx+1)..=*eidx {
                    bsum += height[i];
                }
                res += (eidx - sidx) as i32 * h - bsum;
                sidx = *eidx;
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_42() {
        assert_eq!(Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
        assert_eq!(Solution::trap(vec![4,2,0,3,2,5]),9);
    }
}
