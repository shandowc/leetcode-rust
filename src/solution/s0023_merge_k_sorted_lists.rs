/**
 * [23] 合并 K 个升序链表
 *
 * 给你一个链表数组，每个链表都已经按升序排列。
 * 请你将所有链表合并到一个升序链表中，返回合并后的链表。
 *  
 * 示例 1：
 * 输入：lists = [[1,4,5],[1,3,4],[2,6]]
 * 输出：[1,1,2,3,4,4,5,6]
 * 解释：链表数组如下：
 * [
 *   1->4->5,
 *   1->3->4,
 *   2->6
 * ]
 * 将它们合并到一个有序链表中得到。
 * 1->1->2->3->4->4->5->6
 *
 * 示例 2：
 * 输入：lists = []
 * 输出：[]
 *
 * 示例 3：
 * 输入：lists = [[]]
 * 输出：[]
 *
 *  
 * 提示：
 *
 * 	k == lists.length
 * 	0 <= k <= 10^4
 * 	0 <= lists[i].length <= 500
 * 	-10^4 <= lists[i][j] <= 10^4
 * 	lists[i] 按 升序 排列
 * 	lists[i].length 的总和不超过 10^4
 *
 */
pub struct Solution {}

use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.cn/problems/merge-k-sorted-lists/
// discuss: https://leetcode.cn/problems/merge-k-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::BinaryHeap;

impl PartialOrd for Box<ListNode> {
    fn partial_cmp(&self, other: &Box<ListNode>) -> Option<std::cmp::Ordering> {
        if self.val < other.val {
            Some(std::cmp::Ordering::Greater)
        } else if self.val == other.val {
            Some(std::cmp::Ordering::Equal)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

impl Ord for Box<ListNode> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.val < other.val {
            std::cmp::Ordering::Greater
        } else if self.val == other.val {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Less
        }
    }
}

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for p in lists {
            if let Some(e) = p {
                heap.push(e)
            }
        }
        let mut dummy_head = Some(Box::new(ListNode { next: None, val: 0 }));
        let mut curr = &mut dummy_head;
        loop {
            let mut p = heap.pop();
            if p.is_none() {
                break;
            }
            let mut p = p.unwrap();

            let next = p.next.take();
            if let Some(next) = next {
                heap.push(next);
            }
            curr.as_mut().unwrap().next = Some(p);
            curr = &mut curr.as_mut().unwrap().next;
        }
        dummy_head.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_23() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6]),
            ],),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]), None);
        assert_eq!(Solution::merge_k_lists(vec![None]), None);
    }
}
