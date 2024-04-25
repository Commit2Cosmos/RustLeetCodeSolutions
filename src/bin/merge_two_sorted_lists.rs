pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>, 
        list2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(l2)) => Some(l2),
            (Some(l1), None) => Some(l1),
            (Some(l1), Some(l2)) => {
                if l1.val <= l2.val {
                    Some(Box::new(ListNode { 
                        val: l1.val, 
                        next: Self::merge_two_lists(l1.next, Some(l2)) 
                    }))
                } else {
                    Some(Box::new(ListNode { 
                        val: l2.val, 
                        next: Self::merge_two_lists(Some(l1), l2.next) 
                    }))
                }
            },
        }
    }

    pub fn merge_two_lists_2(
        mut list1: Option<Box<ListNode>>, 
        mut list2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut pointer = &mut list1;

        while list2.is_some() {
            if pointer.is_none() || pointer.as_ref()?.val > list2.as_ref()?.val {
                std::mem::swap(pointer, &mut list2)
            }
            pointer = &mut pointer.as_mut()?.next;
        }
        list1
    }

}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
        next: None,
        val
        }
    }
}

fn main() {
    let list1 = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode::new(4))) })) }));
    let list2 = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 3, next: Some(Box::new(ListNode::new(4))) })) }));
    println!("{:?}", Solution::merge_two_lists_2(list1, list2))
}

