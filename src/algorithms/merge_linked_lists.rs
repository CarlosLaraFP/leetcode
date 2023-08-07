/// You are given the heads of two sorted linked lists list1 and list2.
///
/// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
///
/// Return the head of the merged linked list.
///
pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    ListNode::from_array(vec![1, 1, 2, 3, 4, 4])
}

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

    fn from_array(array: Vec<i32>) -> Option<Box<Self>> {
        if array.is_empty() { return None; }

        let mut array = array.into_iter();
        let mut head = Some(Box::new(Self::new(array.next().unwrap())));
        let mut current = &mut head;

        while let Some(v) = array.next() {
            let mut value = current.as_mut().unwrap();
            value.next = Some(Box::new(Self::new(v)));
            current = &mut value.next;
        }

        head
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
}
