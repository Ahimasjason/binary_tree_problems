
use std::hash::Hash;
use std::collections::HashSet;

use super::{Node, BT};





impl <T> BT <T> 
    where  T : std::fmt::Debug + Hash + Eq + Copy

{
    // If duplicates value exists in tree returns True
    fn _has_duplicate<'a>(root: &'a Option<Box<Node<T>>>, set : &mut HashSet<&'a T>) -> bool {
        
        if root.is_none(){ return false }
        
        if set.contains(&root.as_ref().unwrap().data){
            return true
        }
        set.insert(&root.as_ref().unwrap().data);

        
        Self::_has_duplicate(&root.as_ref().unwrap().left, set) || Self::_has_duplicate(&root.as_ref().unwrap().right, set)
    
    }

    fn has_duplicate(&self) -> bool {

        Self::_has_duplicate(&self.root, &mut HashSet::new())
    }
}




#[cfg(test)]
mod tests {

    use super::*;

    
    
    fn build_non_dup_node() -> Option<Box<Node<usize>>>{

        let mut vals = [Some(0),Some(1),None,Some(3),None, None, Some(2), None, None];

        crate::sub_tree_of_another::tests::build_from_none(&mut vals.iter())
    }

    fn build_dup_node() -> Option<Box<Node<usize>>>{

        let mut vals = [Some(0),Some(1),None,Some(3),None, None, Some(1), None, None];

        crate::sub_tree_of_another::tests::build_from_none(&mut vals.iter())
    }


    #[test]
    fn test_duplicate(){ 
        let non_dup = BT{root:build_non_dup_node()};
        let dup = BT{root:build_dup_node()};
        assert!(!non_dup.has_duplicate());
        assert!(dup.has_duplicate());
    }
}