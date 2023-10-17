
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

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> 
    Option<Box<ListNode>>{
        let mut head = ListNode::new(0);
        let mut curr = & mut head;
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() {
            println!("{:?} {:?}\n",l1,l2);
            let mut sum = carry;
            
            if let Some(node) = l1{
                sum += node.val;
                l1 = &node.next;
            }

            if let Some(node) = l2{
                sum += node.val;
                l2 = &node.next;
            }
            let val = sum % 10;
            carry = sum / 10;
                    
            curr.next = Some(Box::new(ListNode::new(val)));
            curr = curr.next.as_mut().unwrap();

 

        }
        if carry > 0{
            curr.next = Some(Box::new(ListNode::new(carry)));
            curr = curr.next.as_mut().unwrap();
        }

        Some(Box::new(*head.next.unwrap()))
    }
}



fn main() {
    let l1 = Some(Box::new(ListNode::new(2)));
    let l2 = Some(Box::new(ListNode::new(5)));
    Solution::add_two_numbers(l1, l2);
}
