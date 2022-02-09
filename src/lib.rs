
#[allow(unused, dead_code)]

use std::collections::{
    VecDeque,
    HashMap,
};
pub mod bst;
pub mod build;
pub mod node_sum;
pub mod sub_tree_of_another;
pub mod duplicate;
pub mod nodes_at_k_dist;
pub mod nodes_at_dist_k;
pub mod anc_nodes;
pub mod liss;
pub mod random_pointers;
pub mod serde;
pub mod enc_dec;
pub mod zig_zag;
pub mod isomorphic;
pub mod dll;
pub mod skewed;
pub mod threaded_bt;



#[derive(Debug, PartialEq, Clone,Hash, Eq)]
struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}



impl <T> Node<T>
    where  T: std::fmt::Debug + Copy + Ord + std::hash::Hash
{
    fn new(data: T) -> Option<Box<Node<T>>>{

        Some(
            Box::new(Node{data: data,left: None, right: None})
        )
    }

}
#[derive(Debug)]
struct BT<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Debug + Copy + Ord + std::hash::Hash> BT<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn in_traverse(&self) {
        let mut list = vec![];
        let mut root = self.root.as_ref();
        while !list.is_empty() || root.is_some() {
            if root.is_some() {
                list.push(root);
                root = root.unwrap().left.as_ref();
            } else {
                let t = list.pop().unwrap();
                println!("data : {:?}", t.unwrap().data);
                root = t.unwrap().right.as_ref();
            }
        }
    }

    fn preorder(&self){

        let mut stack = Vec::new();
        stack.push(self.root.as_ref());
        while !stack.is_empty(){
            let current = stack.pop().unwrap();
            if current.is_some(){
                println!("PreOrder: {:?}", current.unwrap().data);
                stack.push(current.unwrap().right.as_ref());
                stack.push(current.unwrap().left.as_ref());    
            }
            
        }
    }

    fn post_order(&self){

        let mut root = self.root.as_ref();
        let mut set = std::collections::HashSet::new();

        while root.is_some() && !set.contains(root.unwrap()){
            
            if root.unwrap().left.is_some() && !set.contains(root.unwrap().left.as_ref().unwrap()){
                root = root.unwrap().left.as_ref();
            }else if root.unwrap().right.is_some() && !set.contains(root.unwrap().right.as_ref().unwrap()){
                root = root.unwrap().right.as_ref();
            } else {
                set.insert(root.unwrap());
                println!("Post Order -> {:?}", root.unwrap().data );
                root = self.root.as_ref();

            }

        }
    }

    // fn _remove(node : &mut Box<Node<T>>, key : T) -> Option<Node<T>>{

    
    //     let mut list = vec![Some(node)];

    //     while !list.is_empty() {
    //         let item = list.pop().unwrap();
    //         match item {
    //             Some(n) => {


    //                 if n.left.as_ref().unwrap().data == key {

    //                     let lnode = n.left.take();
    //                     if lnode.left.is_none() && lnode.right.is_none() {
    //                         // leaf node
    //                         n.left = None;
    //                     } else if lnode.left.is_none() && lnode.right.is_some(){
    //                         // right node has child
    //                         n.left = lnode.right;
    //                     } else if lnode.left.is_some() && lnode.right.is_none(){
    //                         // left node has child
    //                         n.left = lnode.left;
    //                     } else {
    //                         // node has both child
                            
    //                         let rightdata = lnode.as_ref().unwrap().right.unwrap().data;
    //                         lnode.data = rightdata;
    //                         n.left = lnode;
    //                     }
                        
    //                 }

    //                 if n.right.as_ref().data == key {
    //                     let lnode = n.right.take();
    //                     if lnode.left.is_none() && lnode.right.is_none() {
    //                         n.left = None;
    //                         return lnode
    //                     }
    //                 }
    //                 if (node.left.is_none() || node.right.is_none()){
    //                     //  both leaf nodes are none
    //                 }

    //                 //  only left node is not empty
    //                 if (node.left.is_some() && node.right.is_none()){

    //                     if node.left.as_ref().unwrap().data == key {
    //                         let lnode = node.left.take().unwrap();
    //                         node.left.as_mut().left = lnode

    //                     }
    //                 }

    //                 // left node is empty
    //                 if (node.left.is_none() && node.right.is_some()){

    //                 } 



    //             }
    //             Some(n) => {
    //                 list.push(n.right.as_mut());
    //                 list.push(n.left.as_mut());
    //             }
    //             _ => {}
    //         }
    //     }
    //     None
    // }

    
    // fn delete(&mut self, key : T) Option<Node<T>>{

    //     if self.root.is_none(){
    //             return None
    //     }
    //     if self.root.left.is_none() || self.root.right.is_none() {
    //         if self.root.as_ref().unwrap().data == key {
    //             return self.root.take();

    //         } else {
    //             None
    //         }
    //     }


    //     let mut key_node = None;
    //     let l_node = None;
    //     let mut queue = VecDeque::new();
    //     queue.push_back(self.root.as_deref_mut());
    //     while !queue.is_empty(){

    //         let curr_node = queue.pop_front().unwrap();
            
    //         if *curr_node.as_ref()
    //         // 
    //     }

    //     None
    // }
    
    fn _remove(node : Option<Box<&mut Node<T>>> , key : T) -> Option<Box<Node<T>>>{

        if node.is_none(){
            return None
        }

        None   
    }

    
    // fn _delete(&mut self, mut key : T) -> Option<T>{

    //     if self.root.is_none() {
    //         return None
    //     }
        
    //     // let mut lookup_root = self.root.as_mut();
    //     let mut data = None;
    //     if self.root.as_ref().unwrap().data == key {
    //         //  delete the root
    //         // let root = self.root.take();
    //         data  = Some(self.root.as_ref().unwrap().data);
    //         if self.root.as_ref().unwrap().left.is_none(){
    //             // roots left node is none so make the right node as root
    //             let root = self.root.take();

    //             self.root  = root.unwrap().right.take();

    //         }else if self.root.as_ref().unwrap().right.is_none(){
    //             let root = self.root.take();
    //             self.root = root.unwrap().left.take();
    //         } else {

    //             let right_data = self.root.as_ref().unwrap().right.as_ref().unwrap().data;
    //             self.root.as_mut().unwrap().data = right_data;


    //             // lookup_root = 
                
    //         }
    //     }

    //     let mut deque = VecDeque::new();
    //     deque.push_back(self.root.as_ref());

    //     while !deque.is_empty() {

    //         let node = deque.pop_front().unwrap();
    //         if node.as_ref().unwrap().data == key {

    //         }else {

    //             i
    //         }
            
    //     }

    //     None
    // }

    fn remove(&mut self, key: T) -> Option<T> {
        if self.root.is_none() {
            return None;
        }   

        let mut node_to_be_delete = None;
        let mut list = vec![self.root.as_mut()];

        while !list.is_empty() {
            let item = list.pop().unwrap();
            match item {
                Some(n) if n.data == key => {
                    node_to_be_delete = Some(n);
                    break;
                }
                Some(n) => {
                    list.push(n.right.as_mut());
                    list.push(n.left.as_mut());
                }
                _ => {}
            }
        }

        None
    }

    fn _height(root : &Option<Box<Node<T>>>) -> isize {

        if root.is_none(){
            return -1;
        }
        let left = Self::_height(&(root.as_ref().unwrap().left));
        let right = Self::_height(&(root.as_ref().unwrap().right));
        if left > right{
            return left + 1;
        }
        right + 1
    }



    fn height(&self) -> isize {

        Self::_height(&self.root)
    }

    fn _is_balanced(root : &Option<Box<Node<T>>>) -> isize{

        if root.is_none() {
            return 0
        }
        let left = Self::_is_balanced(&(root.as_ref().unwrap().left));
        if left == -1 {
            return -1;
        }
        let right = Self::_is_balanced(&(root.as_ref().unwrap().right));
        if right == -1 {
            return -1;
        }

        if (right - left).abs() > 1{
            return -1;
        }

        left.max(right) + 1
    }


    fn is_balanced(&self) -> bool{
        if Self::_is_balanced(&self.root) == -1 {
            false
        } else {
            true
        }
        
    }


    fn level_order(&self){
        let mut queue = VecDeque::new();
        let mut level = vec![None; self.height() as usize  + 2]; // 0 level is is 
        queue.push_back((self.root.as_ref(), 0));
        level[0] = Some(vec![self.root.as_ref().unwrap().data]);

        while !queue.is_empty() {

            let (node, lvl) = queue.pop_front().unwrap();
            if node.is_some(){
                queue.push_back((node.as_ref().unwrap().left.as_ref(), lvl + 1));
                queue.push_back((node.as_ref().unwrap().right.as_ref(), lvl + 1));
                println!("lvl {}  len {}", lvl, level.len());
                if level[lvl + 1 ].is_none(){
                    let mut v = vec![];
                    node.as_ref().map(|n|{
                        if n.left.is_some(){
                            v.push(n.left.as_ref().unwrap().data);
                        }
                        if n.right.is_some(){
                            v.push(n.right.as_ref().unwrap().data);
                        } 
                    });
                    level[lvl +1] = Some(v);
                } else {
                    level[lvl + 1].as_mut().map(|n| {
                           node.as_ref().map(|ref_node|{
                                if ref_node.left.is_some(){
                                    n.push(ref_node.left.as_ref().unwrap().data);
                                }
                                if ref_node.right.is_some(){
                                    n.push(ref_node.right.as_ref().unwrap().data);
                                }
                           });
                    });
                }
            }
        }
        println!("Level : {:?}", level);
        
    }

    /*
     * 1. When the subtrees's both left and right child node values are eual to the root's values
     * 2. when left or right node is null but existing left or right node equal to the root node
     * 3. when leave are null
     * 
     * 
    **/


    fn _unival(root: Option<&Box<Node<T>>>, count: &mut usize) -> bool{

        if root.is_none(){
            return true
        }
        let left = Self::_unival(root.as_ref().unwrap().left.as_ref(), count);
        let right = Self::_unival(root.as_ref().unwrap().right.as_ref(), count);

        if !left || !right {
            return false
        }

        // do the actual unival stuff
        if root.as_ref().unwrap().left.is_some() && 
                                root.as_ref().unwrap().data != root.as_ref().unwrap().left.as_ref().unwrap().data{
                
                return false
        }
        if root.as_ref().unwrap().right.is_some() && 
                                root.as_ref().unwrap().data != root.as_ref().unwrap().right.as_ref().unwrap().data{
                
                return false
        }

        *count += 1;
        true
    }


    fn unival(&self) -> (bool, usize) {

        let mut c = 0;
        (Self::_unival(self.root.as_ref(), &mut c), c)
    }

    fn insert(&mut self, data: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node {
                data: data,
                left: None,
                right: None,
            }));
            return;
        }

        let mut queue = VecDeque::new();
        queue.push_back(self.root.as_mut());

        while !queue.is_empty() {
            // println!("Inside Loop");
            // pop front returns Option<T>
            queue.pop_front().unwrap().map(|node| {
                if node.left.is_some() {
                    queue.push_back(node.left.as_mut());
                } else {
                    node.left = Some(Box::new(Node {
                        data: data,
                        left: None,
                        right: None,
                    }));
                    // println!("RET Left");
                    queue = VecDeque::new();
                    return;
                }

                if node.right.is_some() {
                    queue.push_back(node.right.as_mut());
                } else {
                    node.right = Some(Box::new(Node {
                        data: data,
                        left: None,
                        right: None,
                    }));
                    // println!("RET Right");
                    queue = VecDeque::new();
                    return;
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let mut bt = BT::new();
        // bt.insert(1);
        // bt.insert(2);
        // bt.insert(3);
        // bt.insert(4);
        bt.insert(2);
        bt.insert(3);
        bt.insert(4);
        bt.insert(1);
        bt.insert(5);
        bt.insert(6);
        bt.insert(7);
        bt.insert(8);
        println!("Tree {:?}", bt);
        // bt.in_traverse();
        // bt.post_order();
        // println!("BT {:?}", bt);
        // println!("Removing --> ");
        // bt.preorder();
        bt.level_order();
        assert_eq!(bt.height(), 3);

    }

    // impl <T> Node<T> {
    //     fn new(val : T) -> Link<T>{
    //         Some(Box::new(Self {data: val, left: None, right: None}))
    //     }
    // }

    /**
     *      5
     *    /   \
     *   4     
     *  /
     * 3
     */
    #[test]
    fn test_balanced(){
            let node = Node{
                data: 5,
                left: Some(Box::new(
                                    Node{data:4,
                                         left:Some(Box::new(Node{data:3, left:None,right: None})),
                                         right:None
                                         }
                                         )),
                right: None
            };
            let mut bt = BT{root: Some(Box::new(node))};

            assert!(!bt.is_balanced());

    }




    #[test]
    fn test_unival(){
        let mut bt = BT::new();

        for _ in 0..6{
            bt.insert(5);
        }

        let r = bt.unival();

        eprintln!("{:?}", r);

        assert!(r.0);
        assert_eq!(r.1, 6);
    }

    #[test]
    fn test_height(){

        let mut bt = BT::new();

        for i in 0..6{
            bt.insert(i);
        }

        assert_eq!(2, bt.height());
    }
}
