/**
 * [2] 两数相加
 *
 * 给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。
 * 请你将两个数相加，并以相同形式返回一个表示和的链表。
 * 你可以假设除了数字 0 之外，这两个数都不会以 0 开头。
 *
 * 示例 1：
 * <img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/01/02/addtwonumber1.jpg" style="width: 483px; height: 342px;" />
 * 输入：l1 = [2,4,3], l2 = [5,6,4]
 * 输出：[7,0,8]
 * 解释：342 + 465 = 807.
 *
 * 示例 2：
 *
 * 输入：l1 = [0], l2 = [0]
 * 输出：[0]
 *
 * 示例 3：
 *
 * 输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * 输出：[8,9,9,9,0,0,0,1]
 *
 *
 * 提示：
 *
 * 	每个链表中的节点数在范围 [1, 100] 内
 * 	0 <= Node.val <= 9
 * 	题目数据保证列表表示的数字不含前导零
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.cn/problems/add-two-numbers/
// discuss: https://leetcode.cn/problems/add-two-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

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
    fn add_two_numbers_recursive(
        l1: Option<&Box<ListNode>>,
        l2: Option<&Box<ListNode>>,
        progres: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            if progres > 0 {
                return Some(Box::new(ListNode::new(progres)));
            }
            return None;
        }
        let mut res = l1.map_or(0, |no| no.val) + l2.map_or(0, |no| no.val) + progres;
        let progres = res / 10;
        res = res % 10;

        return Some(Box::new(ListNode {
            val: res,
            next: Self::add_two_numbers_recursive(
                l1.map_or(None, |no| no.next.as_ref()),
                l2.map_or(None, |no| no.next.as_ref()),
                progres,
            ),
        }));
    }

    pub fn add_two_numbers_recursive_loop(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1 = l1;
        let mut p2 = l2;
        let mut progress = 0;

        let mut dummy_head = ListNode::new(0);
        let mut parent = &mut dummy_head;
        loop {
            if p1.is_none() && p2.is_none() && progress == 0 {
                break dummy_head.next;
            }

            let v1 = p1.as_ref().map_or(0, |node| node.val);
            let v2 = p2.as_ref().map_or(0, |node| node.val);
            let mut res = v1 + v2 + progress;
            progress = res / 10;
            res = res % 10;

            let mut node = Box::new(ListNode::new(res));
            parent.next = Some(Box::clone(&node));

            p1 = p1.map_or(None, |node| node.next);
            p2 = p2.map_or(None, |node| node.next);

            parent = &mut node;
        }
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two_numbers_recursive_loop(l1, l2)
        // Self::add_two_numbers_recursive(l1.as_ref(), l2.as_ref(), 0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}
