/**
 * [24] 两两交换链表中的节点
 *
 * 给你一个链表，两两交换其中相邻的节点，并返回交换后链表的头节点。你必须在不修改节点内部的值的情况下完成本题（即，只能进行节点交换）。
 *  
 * 示例 1：
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/swap_ex1.jpg" style="width: 422px; height: 222px;" />
 * 输入：head = [1,2,3,4]
 * 输出：[2,1,4,3]
 *
 * 示例 2：
 *
 * 输入：head = []
 * 输出：[]
 *
 * 示例 3：
 *
 * 输入：head = [1]
 * 输出：[1]
 *
 *  
 * 提示：
 *
 * 	链表中节点的数目在范围 [0, 100] 内
 * 	0 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.cn/problems/swap-nodes-in-pairs/
// discuss: https://leetcode.cn/problems/swap-nodes-in-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { next: head, val: 0 }));
        let mut p = &mut dummy_head;
        loop {
            if p.as_ref().is_none()
                || p.as_ref().unwrap().next.as_ref().is_none()
                || p.as_ref()
                    .unwrap()
                    .next
                    .as_ref()
                    .unwrap()
                    .next
                    .as_ref()
                    .is_none()
            {
                break;
            }
            let mut m = p.as_mut().unwrap().next.take();
            let mut n = m.as_mut().unwrap().next.take();
            let mut x = n.as_mut().unwrap().next.take();
            m.as_mut().unwrap().next = x;
            n.as_mut().unwrap().next = m;
            p.as_mut().unwrap().next = n;
            p = &mut p.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        dummy_head.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_24() {
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4])),
            to_list(vec![2, 1, 4, 3])
        );
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4, 5])),
            to_list(vec![2, 1, 4, 3, 5])
        );
        assert_eq!(Solution::swap_pairs(to_list(vec![])), None);
        assert_eq!(Solution::swap_pairs(to_list(vec![1])), to_list(vec![1]));
    }
}
