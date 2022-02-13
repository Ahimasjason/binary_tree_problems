




use crate::bst::search_tree::*;


fn create_sorted_nodes<T:Copy>(nodes_vec : &mut Vec<Option<Box<Node<T>>>>, 
                        mut node : Option<Box<Node<T>>>)
{

    if node.is_none(){return}


   let mut n = node.take().unwrap();
   let left = n.left.take();
   let right = n.right.take();
   create_sorted_nodes(nodes_vec,left);
    nodes_vec.push(Some(n));
    create_sorted_nodes(nodes_vec,right);
}


fn build_balanced<T: Copy>(nodes_vec: &mut Vec<Option<Box<Node<T>>>>, 
                      start : isize, 
                      end:isize
                    ) -> Option<Box<Node<T>>>
{

    if start > end { return None}
    let mid  = (start + end)  / 2;
    if mid < 0 { return None }
    let mut n = nodes_vec[mid as usize].take();
    n.as_mut().unwrap().left = build_balanced(nodes_vec , start, mid -1);
    n.as_mut().unwrap().right = build_balanced(nodes_vec , mid +1, end);
    n
}






#[cfg(test)]
mod tests {

    use super::*;


    fn build_skewed() -> Option<Box<Node<usize>>>{


        let mut tree = Tree::new();

        for i in 1..10{
            tree.insert(i);
        }
        tree.root.take()

    }


    #[test]
    fn test_sorted(){

        let mut vec_node = vec![];

        let n = build_skewed();
        create_sorted_nodes(&mut vec_node, n); 

        eprintln!("{:?}", vec_node);
        let i: isize = vec_node.len() as isize;
        let new_head = build_balanced(&mut vec_node,0,i-1);

        eprintln!("{:?}", vec_node);
        eprintln!("{:?}", new_head);

        
    }
}