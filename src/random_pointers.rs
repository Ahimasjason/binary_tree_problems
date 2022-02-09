

use std::{collections::HashMap, hash::Hash};







type Link<'a, T> = Option<Box<Node<'a, T>>>;
type LinkRef<'a, T> = Option<&'a Box<Node<'a, T>>>;

#[derive(PartialEq, Eq, Hash)]
struct Node<'a, T>{
    data : T,
    left : Link<'a, T>,
    right : Link<'a, T>,
    random : LinkRef<'a, T>,
}



impl <'a, T>  Node <'a, T> 

    where  
        T : Copy + Ord + Hash

{

    fn new(data: T) -> Link<'a, T> { 

        Some(Box::new(Node{data, left: None, right: None , random: None}))
    }


    fn copy_node(root: Option<&Box<Node<T>>>) -> Option<Box<Node<'a, T>>>
    
    {

        if root.is_none(){ return None}
        let mut new = Node::new(root.as_ref().unwrap().data);
        new.as_mut().unwrap().left = Node::copy_node(root.as_ref().unwrap().left.as_ref());
        new.as_mut().unwrap().right = Node::copy_node(root.as_ref().unwrap().right.as_ref());
        new
    }

    // fn copy_pointers<'b>(root: Option<&'a Box<Node<T>>>, target : Option<&'b Box<Node<T>>> , map : &mut HashMap<Option<&'a Box<Node<T>>>, Option<&'b Box<Node<T>>>>) {


    //     if root.is_none() {return}
    //     map.insert(root, target);
    //     Node::copy_pointers(
    //         root.as_ref().unwrap().left.as_ref(), target.as_ref().unwrap().left.as_ref(), map
    //     );
    //     Node::copy_pointers(
    //         root.as_ref().unwrap().right.as_ref(), target.as_ref().unwrap().right.as_ref(), map
    //     );
    // }

    // fn new_copy(mut root: Option<&Box<Node<T>>>){ 

    //     let mut copied = Node::copy_node(root);
    //     let mut map = HashMap::new();
    //     // Node::copy_pointers(root, copied.as_ref(), &mut map);
    //     {
    //         let mut stack = Vec::new();
    //         stack.push((root, copied.as_ref()));

    //         while !stack.is_empty() {

    //             let (n,c) = stack.pop().unwrap();
    //             if n.is_some() && c.is_some(){ 
    //                 map.insert(n,c);

    //                 stack.push(
    //                     (n.unwrap().right.as_ref(), c.unwrap().right.as_ref())
    //                 );
    //                 stack.push(
    //                     (n.unwrap().left.as_ref(), c.unwrap().left.as_ref())
    //                 );
    //             }
    //         }
    //     }
        
    //     {
    //         let mut stack = Vec::new();
    //     stack.push(
    //         (root, copied.as_mut())
    //     );

    //     while !stack.is_empty() {
    //         let curr = stack.pop().unwrap();
    //         if curr.0.is_some(){ 
    //             // let curr_rand = map.get(&curr.0);
    //             let curr_ptr = curr.0;
    //             let mut copy_ptr = curr.1;
    //             let curr_rand = *map.get(&curr_ptr.unwrap().random.as_deref()).unwrap();
    //             copy_ptr.as_mut().unwrap().random = curr_rand;    

                

    //         }
    //     }
    //     }

        
    // }



}





// struct Map<'a,T>(HashMap<Option<&'a Box<Node<'a, T>>>, Option<&'a Box<Node<'a, T>>>>);




// impl<'a,T> Map <'a,T> 

//     where 
//         T: Hash + Eq + Ord
// {
//     fn insert(&mut self, node1: Option<&'a Box<Node<T>>>, node2: Option<&'a Box<Node<T>>>){

//         self.0.insert(node1, node2);
//     }
// }   






#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_works(){

        assert!(true);
    }
}