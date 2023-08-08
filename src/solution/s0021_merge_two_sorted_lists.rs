/**
 * [21] 合并两个有序链表
 *
 * 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 
 *  
 * 示例 1：
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg" style="width: 662px; height: 302px;" />
 * 输入：l1 = [1,2,4], l2 = [1,3,4]
 * 输出：[1,1,2,3,4,4]
 * 
 * 示例 2：
 * 
 * 输入：l1 = [], l2 = []
 * 输出：[]
 * 
 * 示例 3：
 * 
 * 输入：l1 = [], l2 = [0]
 * 输出：[0]
 * 
 *  
 * 提示：
 * 
 * 	两个链表的节点数目范围是 [0, 50]
 * 	-100 <= Node.val <= 100
 * 	l1 和 l2 均按 非递减顺序 排列
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.cn/problems/merge-two-sorted-lists/
// discuss: https://leetcode.cn/problems/merge-two-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode {
            val: 0,
            next: None,
        }));
        let mut p = &mut dummy_head;
        let mut p1 = list1;
        let mut p2 = list2;
        while p1.is_some() || p2.is_some() {
            if p1.is_none() {
                p.as_mut().unwrap().next = p2;
                break;
            } else if p2.is_none() {
                p.as_mut().unwrap().next = p1;
                break;
            }
            let next = if p1.as_ref().unwrap().val > p2.as_ref().unwrap().val {
                let n = p2.as_mut().unwrap().next.take();
                let next = p2.take();
                p2 = n;
                next
            } else {
                let n = p1.as_mut().unwrap().next.take();
                let next = p1.take();
                p1 = n;
                next
            };
            p.as_mut().unwrap().next = next;
            p = &mut p.as_mut().unwrap().next;
        }
        dummy_head.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_21() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
    }
}
