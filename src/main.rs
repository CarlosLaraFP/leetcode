use leetcode::algorithms::merge_linked_lists::ListNode;

fn main() {
    let a = ListNode::from_array(vec![1, 2, 4]);
    a.as_ref().unwrap().display();
    let b = ListNode::from_array(vec![1, 3, 4]);
    b.as_ref().unwrap().display();
    let result = ListNode::from_array(vec![1, 1, 2, 3, 4, 4]);
    result.as_ref().unwrap().display();
}
