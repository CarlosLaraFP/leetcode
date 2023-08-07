/// You are given the heads of two sorted linked lists list1 and list2.
///
/// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
///
/// Return the head of the merged linked list.
///
/// No recursion and therefore O(1) space complexity.
/// No memory allocations --> Splicing as required by problem description
/// Optimal run-time complexity O(n).
///
pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut r = &mut list1;

    while list2.is_some() {
        if r.is_none() || list2.as_ref()?.val < r.as_ref()?.val {
            std::mem::swap(r, &mut list2);
        }
        r = &mut r.as_mut()?.next;
    }

    list1
}

/*
    match (list1, list2) {
        (Some(l), None) | (None, Some(l)) => Some(l),
        (None, None) => None,
        (Some(l1), Some(l2)) => match l1.val <= l2.val {
            true  => Some(Box::new(ListNode {
                val: l1.val,
                next: merge_two_lists(l1.next, Some(l2)),
            })),
            false => Some(Box::new(ListNode {
                val: l2.val,
                next: merge_two_lists(Some(l1), l2.next),
            })),
        },
    }
 */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None
        }
    }

    pub fn from_array(array: Vec<i32>) -> Option<Box<Self>> {
        if array.is_empty() { return None; }

        let mut array = array.into_iter();
        let mut head = Some(Box::new(Self::new(array.next()?)));
        let mut current = &mut head;

        while let Some(v) = array.next() {
            let mut value = current.as_mut()?;
            value.next = Some(Box::new(Self::new(v)));
            current = &mut value.next;
        }

        head
    }

    pub fn display(&self) {
        println!("{}", self.val);
        let mut current = &self.next;
        while let Some(v) = current {
            println!("{}", v.val);
            current = &v.next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let a = ListNode::from_array(vec![1, 2, 4]);
        let b = ListNode::from_array(vec![1, 3, 4]);
        let result = ListNode::from_array(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(merge_two_lists(a, b), result);
    }

    #[test]
    fn second() {
        let a = ListNode::from_array(vec![]);
        let b = ListNode::from_array(vec![0]);
        let result = ListNode::from_array(vec![0]);
        assert_eq!(merge_two_lists(a, b), result);
    }

    #[test]
    fn third() {
        let a = ListNode::from_array(vec![]);
        let b = ListNode::from_array(vec![]);
        let result = ListNode::from_array(vec![]);
        assert_eq!(merge_two_lists(a, b), result);
    }
}
