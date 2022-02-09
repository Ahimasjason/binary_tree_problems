

use std::rc::Rc;
use std::cell::RefCell;


type BareNode<T> = Rc<RefCell<Node<T>>>;
type Tree<T> = Option<BareNode<T>>;

pub struct Node<T>{

    pub data : T,
    left : Tree<T>,
    right : Tree<T>,
    left_thread : bool,
    right_thread : bool
}




impl<T> Node <T> 

    where 
        T : Copy + Ord
{

    pub fn new(data : T) -> Tree<T> {

        Some (
            Rc::new(
                RefCell::new(
                    Node {data, left: None, right: None , left_thread:false, right_thread: false}
                )
            )
        )
    }
}


pub struct ThreadTree<T>
{
    pub root : Tree<T>
}

impl <T> ThreadTree <T>
    where  T : std::fmt::Debug + Ord + Copy
{

    fn new() -> Self 
    {
        Self { root : None}
    }

    // fn insert(&mut self, data: T) 
    // {
    //     let mut ptr = self.root.as_ref();

    //     let mut parent = None;

    //     while let Some(p) = ptr {

    //         let curr_data = p.clone();
    //         let (node_data, lt, rt) = {
    //             (curr_data.borrow().data, curr_data.borrow().left_thread , curr_data.borrow().right_thread)
    //         };
    //         if node_data == data {
    //             return;
    //         }

    //         parent = Some(curr_data);

    //         if node_data < data {

    //             if !lt{
    //                 ptr  = {
    //                     let tmp = p.borrow().left.clone();
    //                     tmp.as_ref()
    //                 };

    //             } else {
    //                 break
    //             }
    //         }


    //     }



    // }
}
