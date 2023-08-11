/**
 * [25] K 个一组翻转链表
 *
 * 给你链表的头节点 head ，每 k 个节点一组进行翻转，请你返回修改后的链表。
 * k 是一个正整数，它的值小于或等于链表的长度。如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。
 * 你不能只是单纯的改变节点内部的值，而是需要实际进行节点交换。
 *  
 * 示例 1：
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/reverse_ex1.jpg" style="width: 542px; height: 222px;" />
 * 输入：head = [1,2,3,4,5], k = 2
 * 输出：[2,1,4,3,5]
 *
 * 示例 2：
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/reverse_ex2.jpg" style="width: 542px; height: 222px;" />
 *
 * 输入：head = [1,2,3,4,5], k = 3
 * 输出：[3,2,1,4,5]
 *
 *  
 * 提示：
 *
 * 	链表中的节点数目为 n
 * 	1 <= k <= n <= 5000
 * 	0 <= Node.val <= 1000
 *
 *  
 * 进阶：你可以设计一个只用 O(1) 额外内存空间的算法解决此问题吗？
 *
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.cn/problems/reverse-nodes-in-k-group/
// discuss: https://leetcode.cn/problems/reverse-nodes-in-k-group/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k < 2 {
            return head;
        }
        let mut dummy_head = Box::new(ListNode { val: 0, next: head });
        unsafe {
            let mut fast = &mut dummy_head as *mut Box<ListNode>;
            let mut head = &mut dummy_head as *mut Box<ListNode>;
            let mut m: Option<Box<ListNode>> = None;
            'outer: loop {
                for _ in (0..k) {
                    if (*fast).next.as_ref().is_none() {
                        break 'outer;
                    }
                    fast = (*fast).as_mut().next.as_mut().unwrap();
                }
                m = (*head).as_mut().next.take();
                for _ in (0..k) {
                    let n = m.as_mut().unwrap().next.take();
                    m.as_mut().unwrap().next = (*head).as_mut().next.take();
                    (*head).as_mut().next = m;
                    m = n;
                }
                for _ in 0..k {
                    head = (*head).as_mut().next.as_mut().unwrap();
                }
                head.as_mut().unwrap().next = m;
                fast = head;
            }
        }
        dummy_head.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_25() {
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1]), 1),
            to_list(vec![1])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5, 6]), 3),
            to_list(vec![3, 2, 1, 6, 5, 4])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 3),
            to_list(vec![3, 2, 1, 4, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![2, 1, 4, 3, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2]), 2),
            to_list(vec![2, 1])
        );
    }
}
