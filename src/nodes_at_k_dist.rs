
//! Given the root of the tree and int `k` findout 
//! all the nodes which are at distance k from the root
//! 
//! 
//! 



use super::*;

impl <T> BT <T> 
    where T : std::fmt::Debug + Copy

{

    fn _distance_at_k(root: &Option<Box<Node<T>>>, k: isize, v : &mut Vec<T>) -> Vec<T> {
        if root.is_none(){
            return v.to_vec()
        }
        if k == 0{
            v.push(root.as_ref().unwrap().data);
            return v.to_vec()
        }
        Self::_distance_at_k(&root.as_ref().unwrap().left,  k -1 , v);
        Self::_distance_at_k(&root.as_ref().unwrap().right, k -1 , v);
        v.to_vec()
    }


    fn distance_at_k(&self, mut k: isize) -> Vec<T>{
        let mut v = vec![];
        Self::_distance_at_k(&self.root, k ,&mut v)
    }

}



#[cfg(test)]
mod tests {
    use super::*;


    fn build_node() -> Option<Box<Node<usize>>>{

        let mut vals = [
            Some(1),Some(2),Some(4),None,Some(7),None, Some(3), Some(5),None, None,Some(6),None, None
        ];

        crate::sub_tree_of_another::tests::build_from_none(&mut vals.iter())
    }


    #[test]
    fn test_dist(){
        let bt = BT{root: build_node()};
        println!("{:?}", bt.distance_at_k(2));
    }

}