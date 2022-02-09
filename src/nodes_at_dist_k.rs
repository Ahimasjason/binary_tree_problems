use super::{Node,BT};

use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash};

use std::collections::hash_map::Entry;


/**       
 *            20
 *           /  \
 *          8    22
 *         / \
 *        4   12
 *           /  \
 *          10   14
 * 
 * 
 * 
 * */ 


// {Some(Node { data: 14, left: None, right: None }): [Some(Node { data: 20, left: Some(Node { data: 8, left: Some(Node { data: 4, left: None, right: None }), right: Some(Node { data: 12, left: None, right: None }) }), right: Some(Node { data: 14, left: None, right: None }) }), Some(Node { data: 20, left: Some(Node { data: 8, left: Some(Node { data: 4, left: None, right: None }), right: Some(Node { data: 12, left: None, right: None }) }), right: Some(Node { data: 14, left: None, right: None }) })], Some(Node { data: 8, left: Some(Node { data: 4, left: None, right: None }), right: Some(Node { data: 12, left: None, right: None }) }): [Some(Node { data: 20, left: Some(Node { data: 8, left: Some(Node { data: 4, left: None, right: None }), right: Some(Node { data: 12, left: None, right: None }) }), right: Some(Node { data: 14, left: None, right: None }) }), Some(Node { data: 20, left: Some(Node { data: 8, left: Some(Node { data: 4, left: None, right: None }), right: Some(Node { data: 12, left: None, right: None }) }), right: Some(Node { data: 14, left: None, right: None }) }), Some(Node { data: 4, left: None, right: None }), Some(Node { data: 12, left: None, right: None })], Some(Node { data: 20, left: Some(Node { data: 8, left: Some(Node { data: 4, left: None, right: None }), right: Some(Node { data: 12, left: None, right: None }) }), right: Some(Node { data: 14, left: None, right: None }) }): [Some(Node { data: 8, left: Some(Node { data: 4, left: None, right: None }), right: Some(Node { data: 12, left: None, right: None }) }), Some(Node { data: 8, left: Some(Node { data: 4, left: None, right: None }), right: Some(Node { data: 12, left: None, right: None }) }), Some(Node { data: 14, left: None, right: None })], Some(Node { data: 12, left: None, right: None }): [Some(Node { data: 8, left: Some(Node { data: 4, left: None, right: None }), right: Some(Node { data: 12, left: None, right: None }) }), Some(Node { data: 8, left: Some(Node { data: 4, left: None, right: None }), right: Some(Node { data: 12, left: None, right: None }) })], Some(Node { data: 4, left: None, right: None }): [Some(Node { data: 8, left: Some(Node { data: 4, left: None, right: None }), right: Some(Node { data: 12, left: None, right: None }) }), Some(Node { data: 8, left: Some(Node { data: 4, left: None, right: None }), right: Some(Node { data: 12, left: None, right: None }) })]}


fn neighbours<T: std::cmp::Eq + std::hash::Hash>(root: Option<&Box<Node<T>>>) -> HashMap<Option<&Box<Node<T>>>, Vec<Option<&Box<Node<T>>>>>{

    let mut hm = HashMap::new();

    let mut stack = Vec::new();
    stack.push(root);

    while !stack.is_empty() {

        let curr = stack.pop().unwrap();
        if curr.is_some(){
            let left = curr.as_ref().unwrap().left.as_ref();
            let right = curr.as_ref().unwrap().right.as_ref();

            if left.is_some(){ 
                hm.entry(curr).or_insert(vec![]).push(left);
                hm.entry(left).or_insert(vec![]).push(curr);

                
                stack.push(left);
            }
            if right.is_some(){ 
                hm.entry(curr).or_insert(vec![]).push(right);
                hm.entry(right).or_insert(vec![]).push(curr);
                stack.push(right);
            }
            
            
        }
    }
    hm
}


fn k_dist_node<T: std::cmp::Eq + std::hash::Hash + Copy>(root: Option<&Box<Node<T>>>,target: Option<&Box<Node<T>>>, k:usize) -> Vec<T>{

        if root.is_none() || target.is_none(){
            return vec![]
        }

        if k == 0 {
            return vec![target.as_ref().unwrap().data]
        }
        let mut queue = VecDeque::new();
        
        let mut visited = HashSet::new();

        let graph = neighbours(root);
        let mut res = vec![];
        queue.push_back((target,0));
        while !queue.is_empty() {

            let (curr ,distance)= queue.pop_front().unwrap();
            visited.insert(curr);
            for neigh in &graph[&curr]{

                if !visited.contains(&neigh){

                        let new_dist = distance + 1;
                        if new_dist == k {
                            res.push(neigh.as_ref().unwrap().data);
                        } else {
                            queue.push_back((*neigh, new_dist));
                        }
                }

            }
            
        }

        res
}

// fn neighbours<T: std::cmp::Eq + std::hash::Hash + Copy +std::fmt::Debug>(root: Option<&Box<Node<T>>>) ->  HashMap<T, Vec<T>>{

//     let mut hm = HashMap::new();

//     let mut stack = Vec::new();
//     stack.push(root);

//     while !stack.is_empty() {

//         let curr = stack.pop().unwrap();
//         if curr.is_some(){
//             let left = curr.as_ref().unwrap().left.as_ref();
//             let right = curr.as_ref().unwrap().right.as_ref();

//             if left.is_some(){ 
//                 hm.entry(curr.unwrap().data).or_insert(vec![]).push(left.unwrap().data);
//                 hm.entry(left.unwrap().data).or_insert(vec![]).push(curr.unwrap().data);

                
//                 stack.push(left);
//             }
//             if right.is_some(){ 
//                 hm.entry(curr.unwrap().data).or_insert(vec![]).push(right.unwrap().data);
//                 hm.entry(right.unwrap().data).or_insert(vec![]).push(curr.unwrap().data);
//                 stack.push(right);
//             }
            
            

//         }
//         eprintln!("{:?}",hm);
//     }
//     hm
// }



fn percolate<T:Ord + Copy + std::fmt::Debug>(node: Option<&Box<Node<T>>>, k : isize){

    if node.is_none() || k < 0 {
        return;
    }
    if k == 0 {
        println!("{:?}", node.as_ref().unwrap().data);
        return
    }

    percolate(node.as_ref().unwrap().left.as_ref(), k - 1);
    percolate(node.as_ref().unwrap().right.as_ref(), k - 1);
}


fn dist<T:Ord + Copy + std::fmt::Debug>(node: Option<&Box<Node<T>>>, target : Option<&Box<Node<T>>>, k: isize) -> isize{

    if node.is_none(){ return - 1 }
    if node == target {
        percolate(node, k);
        return 1
    }

    let left = dist(node.as_ref().unwrap().left.as_ref(), target, k);
    let right = dist(node.as_ref().unwrap().right.as_ref(), target, k);

    if left != -1 {

        if left == k {
            println!("{:?}", node.as_ref().unwrap().data);
        }
        percolate(node.as_ref().unwrap().right.as_ref(), k - left - 1);
        return left + 1
    }

    if right != -1 {

        if right == k {
            println!("{:?}", node.as_ref().unwrap().data);
        }
        percolate(node.as_ref().unwrap().left.as_ref(), k - right - 1);
        return right + 1
    }

    return -1
}



#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_neighbour(){
        let mut vals = [
            Some(1),Some(2),Some(4),None,Some(7),None, Some(3), Some(5),None, None,Some(6),None, None
        ];
        // let mut vals = [
        //     Some(20),Some(8),Some(4),None,None, Some(12),Some(10),None,None, Some(14),None, None,Some(22),None, None
        // ];

        let mut vals  = [
            Some(1),Some(5), Some(12),None,None,Some(7),Some(4),None,None,Some(8),None,None,Some(2),None,Some(5),Some(7),None,None,Some(7)
        ];

        let tree = crate::sub_tree_of_another::tests::build_from_none(&mut vals.iter());
        // eprintln!("{:?}", tree);
        let n = neighbours(tree.as_ref());

        eprintln!("{:?}", n.len());
        assert_eq!(9, n.len());
    }

    #[test]
    fn test_k_dist(){

        let mut vals  = [
            Some(1),Some(5), Some(12),None,None,Some(7),Some(4),None,None,Some(8),None,None,Some(2),None,Some(5),Some(7),None,None,Some(7)
        ];
        
        let tree = crate::sub_tree_of_another::tests::build_from_none(&mut vals.iter()); 

        let  target = tree.as_ref().unwrap().left.as_ref();

        let res = k_dist_node(tree.as_ref(), target,2);

        for r in res{
            assert!(vec![2,4,8].contains(&r));
        } 
        
    }

    #[test]
    fn test_k_dist_percolate(){

        let mut vals  = [
            Some(1),Some(5), Some(12),None,None,Some(7),Some(4),None,None,Some(8),None,None,Some(2),None,Some(5),Some(7),None,None,Some(7)
        ];
        
        let tree = crate::sub_tree_of_another::tests::build_from_none(&mut vals.iter()); 

        let  target = tree.as_ref().unwrap().left.as_ref();

        let res = dist(tree.as_ref(), target,2);

        
        
    }
    
}