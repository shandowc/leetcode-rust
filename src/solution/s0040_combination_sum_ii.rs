/**
 * [40] 组合总和 II
 *
 * 给定一个候选人编号的集合 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。
 * candidates 中的每个数字在每个组合中只能使用 一次 。
 * 注意：解集不能包含重复的组合。
 *  
 * 示例 1:
 *
 * 输入: candidates = [10,1,2,7,6,1,5], target = 8,
 * 输出:
 * [
 * [1,1,6],
 * [1,2,5],
 * [1,7],
 * [2,6]
 * ]
 * 示例 2:
 *
 * 输入: candidates = [2,5,2,1,2], target = 5,
 * 输出:
 * [
 * [1,2,2],
 * [5]
 * ]
 *  
 * 提示:
 *
 * 	1 <= candidates.length <= 100
 * 	1 <= candidates[i] <= 50
 * 	1 <= target <= 30
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/combination-sum-ii/
// discuss: https://leetcode.cn/problems/combination-sum-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates.clone();
        candidates.sort();

        let mut res = vec![];

        let mut sum = candidates[0];
        let mut curr = vec![candidates[0]];
        let mut curr_idx = vec![0];
        let mut last_idx = 0;

        'outer: while curr.len() > 0 {
            // println!("{:?}", curr);
            if sum == target {
                res.push(curr.clone());
            }
            if sum < target {
                let i = last_idx + 1;
                if i < candidates.len() {
                    last_idx = i;
                    curr_idx.push(i);
                    curr.push(candidates[i]);
                    sum += candidates[i];
                    continue;
                }
            }
            'poping: while let Some(v) = curr.pop() {
                last_idx = curr_idx.pop().unwrap();
                sum -= v;

                let mut nexti = last_idx;

                while candidates[nexti] == v {
                    // println!("nexti: {}", nexti);
                    nexti += 1;
                    if nexti == candidates.len() {
                        continue 'poping;
                    }
                }
                last_idx = nexti;
                curr_idx.push(last_idx);
                curr.push(candidates[last_idx]);
                sum += candidates[last_idx];
                continue 'outer;
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
    fn test_40() {
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]],
        );
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
        )
    }
}
