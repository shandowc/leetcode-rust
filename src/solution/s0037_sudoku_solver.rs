/**
 * [37] 解数独
 *
 * 编写一个程序，通过填充空格来解决数独问题。
 * 数独的解法需 遵循如下规则：
 * <ol>
 * 	数字 1-9 在每一行只能出现一次。
 * 	数字 1-9 在每一列只能出现一次。
 * 	数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。（请参考示例图）
 * </ol>
 * 数独部分空格内已填入了数字，空白格用 '.' 表示。
 *  
 * <div class="top-view__1vxA">
 * <div class="original__bRMd">
 * <div>
 * 示例 1：
 * <img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/04/12/250px-sudoku-by-l2g-20050714svg.png" style="height:250px; width:250px" />
 * 输入：board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
 * 输出：[["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
 * 解释：输入的数独如上图所示，唯一有效的解决方案如下所示：
 * <img src=" https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/04/12/250px-sudoku-by-l2g-20050714_solutionsvg.png" style="height:250px; width:250px" />
 *
 *  
 * 提示：
 *
 * 	board.length == 9
 * 	board[i].length == 9
 * 	board[i][j] 是一位数字或者 '.'
 * 	题目数据 保证 输入数独仅有一个解
 * </div>
 * </div>
 * </div>
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/sudoku-solver/
// discuss: https://leetcode.cn/problems/sudoku-solver/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashSet;

impl Solution {
    fn init_sets(board: &mut Vec<Vec<char>>) -> (Vec<HashSet<char>>, Vec<HashSet<char>>, Vec<HashSet<char>>) {
        let (mut rows, mut cols, mut blks) = (vec![], vec![], vec![]);
        for j in 0..9 {
            let mut row = HashSet::new();
            let mut col = HashSet::new();
            let mut blk = HashSet::new();
            rows.push(row);
            cols.push(col);
            blks.push(blk);
        }
        for i in 0..81 {
            let (m, n) = (i/9, i%9);
            let blk_idx = m / 3 * 3 + n / 3;
            match board[m][n] {
                '.' => {},
                c => {
                    cols[n].insert(c);
                    rows[m].insert(c);
                    blks[blk_idx].insert(c);
                },
            }
        }
        return (rows, cols, blks);
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let (mut rows, mut cols, mut blks) = Self::init_sets(board);
        let mut stack = vec![];
        let mut i = 0;
        let mut sc = '1';

        'outer: while i < 81 {
            let (m, n) = (i/9, i%9);
            let blk_idx = m / 3 * 3 + n / 3;
            match board[m][n] {
                '.' => {
                    let mut found = false;
                    for c in sc..='9' {
                        if rows[m].contains(&c) || cols[n].contains(&c) || blks[blk_idx].contains(&c) {
                            continue;
                        }
                        board[m][n] = c;

                        rows[m].insert(c);
                        cols[n].insert(c);
                        blks[blk_idx].insert(c);

                        stack.push((i, c));
                        found = true;
                        break;
                    }
                    if !found {
                        let (mut ni, mut nc) = stack.pop().unwrap();
                        let (m, n) = (ni/9, ni%9);
                        let blk_idx = m / 3 * 3 + n / 3;
                        board[m][n] = '.';
                        rows[m].remove(&nc);
                        cols[n].remove(&nc);
                        blks[blk_idx].remove(&nc);
                        sc = (nc as u8 + 1) as char;
                        i = ni;
                        continue 'outer;
                    }
                },
                _ => {},
            }
            sc = '1';
            i += 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_37() {
        let mut board = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(board, vec![
            vec!['5','3','4','6','7','8','9','1','2'],
            vec!['6','7','2','1','9','5','3','4','8'],
            vec!['1','9','8','3','4','2','5','6','7'],
            vec!['8','5','9','7','6','1','4','2','3'],
            vec!['4','2','6','8','5','3','7','9','1'],
            vec!['7','1','3','9','2','4','8','5','6'],
            vec!['9','6','1','5','3','7','2','8','4'],
            vec!['2','8','7','4','1','9','6','3','5'],
            vec!['3','4','5','2','8','6','1','7','9'],
        ]);
    }
}
