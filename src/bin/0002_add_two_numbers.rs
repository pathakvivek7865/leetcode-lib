// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

type LinkList = Option<Box<ListNode>>;

pub fn from_array(vec: Vec<i32>) -> LinkList {
    let mut outer_node = None;
    
    for &val in vec.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = outer_node;
        
        outer_node = Some(Box::new(node));
    }
    
    outer_node
}

pub fn into_array(list: LinkList) -> Vec<i32> {
    let mut vec = vec![];
    
    let mut list = list;
    while list.is_some() {
        if let Some(value) = list {
            vec.push(value.val);
            list = value.next;
        }
    }
    
    vec.reverse();
    vec
}

struct Solution;
impl Solution {
    pub fn add_two_numbers(l1: LinkList, l2: LinkList) -> LinkList {
        let mut vec1 = into_array(l1);
        let mut vec2 = into_array(l2);

        println!("{:?} {:?}", vec1, vec2);

        let mut result = vec![];
        let mut carry = 0;

        if vec1.len() > vec2.len() {
            for _ in 0..(vec1.len() - vec2.len()) {
                vec2.insert(0, 0);
            }
        } else if vec2.len() > vec1.len() {
            for _ in 0..(vec2.len() - vec1.len()) {
                vec1.insert(0, 0);
            }
        }

        println!("{:?} {:?}", vec1, vec2);

        for i in (0..vec1.len()).rev() {
            let value1 = vec1[i];
            let value2 = vec2[i];

            let sum = (value1 + value2 + carry) as i32;
            println!("{:?}", sum);
            carry = sum / 10;
            result.push(sum % 10);
            if i == 0 && carry != 0 {
                result.push(carry);
            }
        }

        println!("result: {:?}", result);

        from_array(result)
    }
}

fn main() {
    // let val = Solution::add_two_numbers(from_array(vec![2, 4, 3]), from_array(vec![5, 6, 4]));

    let val = Solution::add_two_numbers(from_array(vec![2, 4, 9]), from_array(vec![5, 6, 4, 9]));

    // let val = Solution::add_two_numbers(
    //     from_array(vec![9, 9, 9, 9, 9, 9, 9]),
    //     from_array(vec![9, 9, 9, 9]),
    // );

    println!("{:?}", val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::add_two_numbers(from_array(vec![2, 4, 3]), from_array(vec![5, 6, 4])),
            from_array(vec![7, 0, 8])
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::add_two_numbers(from_array(vec![0]), from_array(vec![0])),
            from_array(vec![0])
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::add_two_numbers(
                from_array(vec![9, 9, 9, 9, 9, 9, 9]),
                from_array(vec![9, 9, 9, 9])
            ),
            from_array(vec![8, 9, 9, 9, 0, 0, 0, 1])
        );
    }

    #[test]
    fn ex4() {
        assert_eq!(
            Solution::add_two_numbers(from_array(vec![2, 4, 9]), from_array(vec![5, 6, 4, 9])),
            from_array(vec![7, 0, 4, 0, 1])
        );
    }
}
