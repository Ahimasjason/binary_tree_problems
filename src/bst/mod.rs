


#![allow(dead_code)]
#![allow(unused_variables)]

pub mod search_tree;
pub mod array_to_bst;
pub mod min_swaps;
pub mod unbalanced_to_balanced;

use std::rc::Rc;
use std::cell::RefCell;



type Link<T> = Option<Box<Node<T>>>;

type NodePtr<T> = Box<Node<T>>;

#[derive(Debug, PartialEq, Clone)]
struct Node<T> {
    data: T,
    left: Link<T>,
    right: Link<T>,
}

#[derive(Debug)]
struct BST<T> {
    root: Link<T>,
    place_holder_low : T,
    place_holder_high : T,

}


impl<T:Ord+ Copy + std::fmt::Debug> BST<T> {

    fn new(place_holder_low: T, place_holder_high: T) -> Self {

        Self{root: None, place_holder_low, place_holder_high}
    }

    fn min_key<'a>(&self, root: &'a Link<T>) -> Option<&'a Box<Node<T>>> {
        let mut current_node = root.as_ref();

        while !self.is_placeholder(current_node.unwrap()) && 
                !self.is_placeholder(current_node.unwrap().left.as_ref().unwrap())
        {
            current_node = current_node.unwrap().left.as_ref();

        }
        current_node
    }


    fn _delete(&mut self, root: Link<T> , key : T) -> Link<T> {

        if self.is_placeholder(root.as_ref().unwrap()){
            return root
        }

        

        match root {

            Some(mut node) => {
                if key < node.data {
                    let left = node.left.take();
                    node.left = self._delete(left, key);
                    return Some(node)

                } else  if key > node.data{
                    let right = node.right.take();
                    node.right = self._delete(right, key);
                    return Some(node)
                } else {

                     if self.is_placeholder(node.left.as_ref().unwrap()){
                         let right = node.right.take();
                         node.left.take();
                         return right
                     } else if self.is_placeholder(node.right.as_ref().unwrap()){
                         let left = node.left.take();
                         node.right.take();
                         return left
                     }else {
                         let minkey = self.min_key(&node.right).unwrap();
                         node.data = minkey.data;
                         node.right = self._delete(node.right, key);
                         return Some(node)

                     }
                }

            },
            _ => {
                //  this will never hit be`cause of placeholder
            },
        }

        None
    }
    fn delete(&mut self, key : T) {

        if self.root.is_none(){
            return;
        }
        let r = self.root.take();
        self.root = self._delete(r, key);
        

    }

    fn _insert(&mut self, mut root: Link<T>, key: T) -> Link<T>{

        //  I should not make use of Option enum 
        if self.is_placeholder(root.as_ref().unwrap()){

            std::mem::forget(root);
            let mut new = self.new_node(key);
            new.as_mut().map(|node| node.left.replace(self.new_node(self.place_holder_low).unwrap()));
            new.as_mut().map(|node| node.right.replace(self.new_node(self.place_holder_high).unwrap()));
            return new
        }
        
        let mut node = std::mem::replace(&mut root, None).unwrap();
        if key < node.data{
            node.left = self._insert(node.left, key);
            // node

        } else {
            node.right = self._insert(node.right, key);
            // node
        }
       
        Some(node)
        }


        


    fn insert(&mut self, key : T) {

        match &self.root {
            Some(_) => {
                let r = std::mem::replace(&mut self.root, None);
                self.root = self._insert(r, key);

            },
            None => {

                let mut new = self.new_node(key);
                new.as_mut().map(|node| node.left.replace(self.new_node(self.place_holder_low).unwrap()));
                new.as_mut().map(|node| node.right.replace(self.new_node(self.place_holder_high).unwrap()));
                // new.as_ref().borrow_mut().right.replace(self.new_node(self.place_holder_high));
                self.root = new;
            }   
        }
    }

    fn inorder(&self){ 

        let mut current_node = self.root.as_ref();
        let mut deque = std::collections::VecDeque::new();
        while !deque.is_empty() || !self.is_placeholder(current_node.unwrap()){
            if !self.is_placeholder(current_node.unwrap()){
                deque.push_back(current_node);
                current_node = current_node.unwrap().left.as_ref();
            } else {
                let tmp = deque.pop_back().unwrap();
                println!("Inorder {:?}", tmp.unwrap().data);
                current_node = tmp.unwrap().right.as_ref();

            }
        }
    }
    
    fn search(&self, key: T) -> bool {

        let mut current_node = self.root.as_ref();
        

        while current_node.is_some() && !self.is_placeholder(current_node.unwrap()) {

            if key < current_node.unwrap().data{
                current_node = current_node.unwrap().left.as_ref();
            } else if key > current_node.unwrap().data {
                current_node = current_node.unwrap().right.as_ref();
            }else {
                return true
            }
                
        }   
        false 
    }
    
    fn new_node(&self, key:T) -> Link<T> {

        Some(
            Box::new(
                    Node {
                        data: key, 
                        left : None,
                        right : None,    
                    }
                )
            )
    }


    
    


    fn is_placeholder(&self, node: &NodePtr<T>) -> bool {

        // let  n = node.borrow();

        node.data == self.place_holder_low || node.data == self.place_holder_high
    }
}



#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_bst(){
        
        let mut bst = BST::new(i32::MIN, i32::MAX);
        // println!("{:?}", bst);
        bst.insert(5);
        bst.insert(6);
        bst.insert(4);
        bst.insert(10);
        bst.insert(12);
        assert!(bst.search(4));
        // println!("Before deleteing 4 {:?}", bst);
        // bst.delete(4);
        // println!("{:?}", bst);
        bst.inorder();
    }
}