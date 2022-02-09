
use std::collections::HashMap;


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

impl<T> Node <T> {

    fn new(data: T) -> Link<T> {
        Some(Box::new(Self {
            data: data,
            left : None,
            right : None,
        }))
    }
}

#[derive(Debug)]
struct BST<T> {
    root: Link<T>,
}

impl<T:Ord+ Copy + std::fmt::Debug + std::hash::Hash> BST<T> {

    fn new() -> Self{
        Self {root:None}
    }


    fn _build(inorder:&HashMap<T,isize>, preorder: &Vec<T>, istart: isize, iend: isize, pre_index: Rc<RefCell<usize>>) -> Link<T> {

        if istart > iend {
            return None
        }
        let pindex = {
            *(pre_index.borrow_mut())
        };
        println!("pindex {:?}", pindex);
        if pindex >= preorder.len(){
            return None
        }
        let index = inorder.get(&preorder[pindex]).unwrap();

        let mut node = Node::new(preorder[pindex]);
        *pre_index.borrow_mut() = pindex + 1;
        
        // get the inorder location
        

        
        // if this is last node in the branch do the early returns
        if istart == iend {
            return node
        }
        // 
        let left = Self::_build(inorder, preorder, istart,index -1, Rc::clone(&pre_index));
        let right = Self::_build(inorder, preorder, index + 1, iend, Rc::clone(&pre_index));
        node.as_mut().map(|n| {
            n.left = left;
            n.right = right
        });
        
    
        node
    }
    
    fn build(inorder: Vec<T>, preorder: Vec<T>) -> Self {

        let map: HashMap<T,isize>  = inorder.into_iter().enumerate().map(|x| (x.1,x.0 as isize)).collect(); 
        let preorder_len = preorder.len() -1 ;
        let state = Rc::new(RefCell::new(0));
        Self{root: Self::_build(&map, &preorder, 0 , preorder_len as isize, Rc::clone(&state))}
    }

    fn _build_post_in(inorder:&HashMap<T,isize>, postorder: &Vec<T>, istart: isize, iend: isize, post_index: Rc<RefCell<usize>>) -> Link<T>{

        if istart > iend {
            return None
        }


        let pindex = {
            *(post_index.borrow_mut())
        };
        println!("pindex {:?} , Start {}, End {} ", pindex, istart, iend);
        
        let index = inorder.get(&postorder[pindex]).unwrap();

        let mut node = Node::new(postorder[pindex]);

        
        
        // get the inorder location
        

        
        // if this is last node in the branch do the early returns
        if istart == iend{
            if pindex != 0 {
                *post_index.borrow_mut() = pindex - 1;        
            }
            return node
        }
        if pindex == 0 {
            return node
        }
        *post_index.borrow_mut() = pindex - 1;
        // 
        let right = Self::_build_post_in(inorder, postorder, index + 1, iend, Rc::clone(&post_index));
        let left = Self::_build_post_in(inorder, postorder, istart,index -1, Rc::clone(&post_index));
        
        node.as_mut().map(|n| {
            n.left = left;
            n.right = right
        });
        
    
        node

    }

    fn build_post_in(inorder: Vec<T>, postorder:Vec<T>) -> Self {
        let map: HashMap<T,isize> = inorder.into_iter().enumerate().map(|x| (x.1, x.0 as isize)).collect();
        let postorder_len = postorder.len() - 1;
        let state = Rc::new(RefCell::new(postorder_len));
        Self { root: Self::_build_post_in(&map, &postorder, 0, postorder_len as isize, Rc::clone(&state))}
    }
}

#[test]
fn test_build_tree(){

    let preorder = [4,2,5,1,6,3].to_vec();
    let inorder = [1,2,4,5,3,6].to_vec();
    let t = BST::build(inorder, preorder);
    // println!("build {:?}", t);
}


#[test]
fn test_build_tree_post(){

    let inorder = [4, 2, 5, 1, 3].to_vec();
    let postorder = [4, 5, 2, 3, 1].to_vec();
    let t = BST::build_post_in(inorder, postorder);
    println!("build {:?}", t);
    let mut stack = Vec::new();
    let mut pre = Vec::new();
    stack.push(t.root.as_ref());
    while !stack.is_empty(){
        let tmp = stack.pop().unwrap();
        if tmp.is_some() {
            pre.push(tmp.unwrap().data);
            stack.push(tmp.unwrap().right.as_ref());
            stack.push(tmp.unwrap().left.as_ref());
        }
    }
    assert_eq!(pre,[1,2,4,5,3]);
}

