/**
 * [19] 删除链表的倒数第 N 个结点
 *
 * 给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。
 *  
 * 示例 1：
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg" style="width: 542px; height: 222px;" />
 * 输入：head = [1,2,3,4,5], n = 2
 * 输出：[1,2,3,5]
 * 
 * 示例 2：
 * 
 * 输入：head = [1], n = 1
 * 输出：[]
 * 
 * 示例 3：
 * 
 * 输入：head = [1,2], n = 1
 * 输出：[1]
 * 
 *  
 * 提示：
 * 
 * 	链表中结点的数目为 sz
 * 	1 <= sz <= 30
 * 	0 <= Node.val <= 100
 * 	1 <= n <= sz
 * 
 *  
 * 进阶：你能尝试使用一趟扫描实现吗？
 * 
 */
pub struct Solution {}
use std::borrow::BorrowMut;

use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.cn/problems/remove-nth-node-from-end-of-list/
// discuss: https://leetcode.cn/problems/remove-nth-node-from-end-of-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode {
            next: head,
            val: 0,
        });
        unsafe {
            let mut slow = &mut dummy_head as *mut Box<ListNode>;
            let mut fast = &mut dummy_head as *mut Box<ListNode>;
            for _ in 0..n {
                fast = (*fast).next.as_mut().unwrap();
            }
            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut().unwrap();
                slow = (*slow).next.as_mut().unwrap();
            }
            (*slow).next = (*slow).next.take().unwrap().next;
        }
        dummy_head.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_19() {
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1), None);
    }
}
