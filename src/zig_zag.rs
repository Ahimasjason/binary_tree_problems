

use super::*;
use std::collections::VecDeque;


fn two_stacks<T: std::fmt::Debug>(node: Option<&Box<Node<T>>>){

    let mut current = Vec::new();
    let mut next = Vec::new();
    current.push(node);
    let mut right_to_left = true;
    while !current.is_empty() {

        let curr = current.pop().unwrap();
        if curr.is_some(){ 
            eprint!("{:?}", curr.unwrap().data);
            match right_to_left {

                true => {
                    next.push(curr.unwrap().left.as_ref());
                    next.push(curr.unwrap().right.as_ref());
                },
                false => {
                    next.push(curr.unwrap().right.as_ref());
                    next.push(curr.unwrap().left.as_ref());
                },
            }
            if current.is_empty() {
                //  now change the state
                right_to_left = ! right_to_left;
                for n in next.drain(..){
                    current.push(n);
                }
            }
        }
    }
}


fn using_deque<T:Copy>(node: Option<&Box<Node<T>>>) -> Vec<T> 
{

    let mut res = Vec::new();
    let mut deque = VecDeque::new();

    deque.push_back(node);
    let mut state = 1;
    while !deque.is_empty()
    {

        
        match state % 2 {

            0 => {
                // right to left
                for n in deque.iter().rev(){
                    if n.is_some(){ 
                        res.push(n.unwrap().data);
                    }
                }
            }, 
            1 => {
                for n in deque.iter(){
                    if n.is_some(){ 
                        res.push(n.unwrap().data);
                    }
                }
            },
            _ => {},
        }

        let mut new = VecDeque::new();
        for n in deque.drain(..){
            if n.is_some(){
                new.push_back(n.unwrap().left.as_ref());
                new.push_back(n.unwrap().right.as_ref());
            }
        }
        deque = new;
        state += 1;
    }
    res
}



fn height<T>(node: Option<&Box<Node<T>>>) -> usize {
    if node.is_none(){ return 0}

    let left = height(node.unwrap().left.as_ref());
    let right = height(node.unwrap().right.as_ref());
    if left > right {left + 1} else {right + 1}
}


fn level<T: std::fmt::Debug>(node: Option<&Box<Node<T>>>, lvl: usize, l : bool){

    if node.is_none(){ return}

    if lvl == 1 {
        println!("{:?}", node.as_ref().unwrap().data);
    }

    if lvl > 1 {

        if l {
            level(node.as_ref().unwrap().left.as_ref(), lvl -1 , l);
            level(node.as_ref().unwrap().right.as_ref(), lvl -1 , l);
        } else {
            level(node.as_ref().unwrap().right.as_ref(), lvl -1 , l);
            level(node.as_ref().unwrap().left.as_ref(), lvl -1 , l);
        }

    }
}

fn zig_zag_traversal<T: std::fmt::Debug>(node: Option<&Box<Node<T>>>){

    let h = height(node);

    let mut l = true;
    for i in 1..=h {
        level(node, i , l);
        l = !l; 
    }
}


fn spiral<T: std::fmt::Debug>(node: Option<&Box<Node<T>>>){

    if node.is_none(){ return}

    let mut stack1 = Vec::new();
    let mut stack2 = Vec::new();
    stack1.push(node);

    while !stack1.is_empty() || !stack2.is_empty() {

        while !stack1.is_empty(){

            let tmp = stack1.pop().unwrap();

            println!("{:?}", tmp.unwrap().data);

            if tmp.unwrap().left.is_some(){
                stack2.push(tmp.unwrap().left.as_ref());
            }
            if tmp.unwrap().right.is_some(){
                stack2.push(tmp.unwrap().right.as_ref());
            }
        }

        while !stack2.is_empty(){

            let tmp = stack2.pop().unwrap();

            println!("{:?}", tmp.unwrap().data);

            if tmp.unwrap().right.is_some(){
                stack1.push(tmp.unwrap().right.as_ref());
            }
            if tmp.unwrap().left.is_some(){
                stack1.push(tmp.unwrap().left.as_ref());
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn build() -> Option<Box<Node<usize>>>{
         let vals  = [
            Some(1),Some(5), Some(6),None,None,Some(7),None,None,Some(2),Some(3),None,None,Some(4),None,None
        ];
        let tree = crate::sub_tree_of_another::tests::build_from_none(&mut vals.iter());
        tree
    }

    #[test]
    fn test_two_stack(){

        let r = build();
        two_stacks(r.as_ref());
    }


    #[test]
    fn test_using_deque(){

        let r = build();
        let res = using_deque(r.as_ref());

        eprintln!("{:?}",res);
    }

    #[test]
    fn test_using_recursion(){

        let r = build();
        // [1, 2, 5, 6, 7, 3, 4]
        zig_zag_traversal(r.as_ref());
    }

    #[test]
    fn test_spiral(){

        let r = build();
        // [1, 2, 5, 6, 7, 3, 4]
        spiral(r.as_ref());
    }

    
}