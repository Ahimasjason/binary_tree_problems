

use super::{BT, Node};




impl <T> BT <T> 
    where T : std::fmt::Debug + Copy + Ord + ToString +std::hash::Hash + std::ops::Add<Output=T> + Default

{

    fn subtree_inorder<'a>(root: &Option<Box<Node<T>>>, char_array : &'a mut String) ->&'a mut String {

        if root.is_none(){
            char_array.push_str("");
            return char_array
        } 

        Self::subtree_inorder(&root.as_ref().unwrap().left, char_array);
        
        char_array.push_str(&root.as_ref().unwrap().data.to_string());

        Self::subtree_inorder(&root.as_ref().unwrap().right, char_array);

        char_array

    }

    fn subtree_preorder<'a>(root : &Option<Box<Node<T>>>, char_array:&'a mut String) -> &'a mut String {

        if root.is_none(){
            char_array.push_str("");
            return char_array
        }


        char_array.push_str(&root.as_ref().unwrap().data.to_string());

        Self::subtree_preorder(&root.as_ref().unwrap().left, char_array);
        Self::subtree_preorder(&root.as_ref().unwrap().right, char_array);
        char_array

    }

    fn _is_subtree(target: &Option<Box<Node<T>>>, source: &Option<Box<Node<T>>>) -> bool {

        if target.is_none(){
            return false;
        }
        if source.is_none(){ return true;}

        
        let mut preorder_str_source = String::new();
        let mut preorder_str_target = String::new();

        Self::subtree_preorder(target, &mut preorder_str_target);
        Self::subtree_preorder(source, &mut preorder_str_source);

        if !preorder_str_target.contains(&preorder_str_target){
            return false
        }

        let mut inorder_str_source = String::new();
        let mut inorder_str_target = String::new();

        Self::subtree_inorder(target, &mut inorder_str_target);
        Self::subtree_inorder(source, &mut inorder_str_source);

        inorder_str_target.contains(&inorder_str_target)
    }

    fn is_subtree(&self, source: &Option<Box<Node<T>>>) -> bool {

        Self::_is_subtree(&self.root, source)

    }

    fn identical(target: &Option<Box<Node<T>>>, source: &Option<Box<Node<T>>>) -> bool {

        if target.is_none() && source.is_none(){ return true}

        if target.is_some() && source.is_some(){ 
            
            target.as_ref().unwrap().data == source.as_ref().unwrap().data && 
                Self::identical(&target.as_ref().unwrap().left, &source.as_ref().unwrap().left) &&
                Self::identical(&target.as_ref().unwrap().right, &source.as_ref().unwrap().right) 
        } else {
            false
        }
    }

    fn _is_subtree_recr(target: &Option<Box<Node<T>>>, source: &Option<Box<Node<T>>>) -> bool {

        if source.is_none(){ return true }
        if target.is_none(){ return false}

        if Self::identical(target, source){return true}


        Self::_is_subtree_recr(&target.as_ref().unwrap().left, source) || Self::_is_subtree_recr(&target.as_ref().unwrap().right, source) 

    }

    fn is_subtree_recr(&self, source: &Option<Box<Node<T>>>) -> bool {

        Self::_is_subtree_recr(&self.root, source)
    }

}



#[cfg(test)]
pub(crate) mod tests{


    use super::*;

    /**
     *  TARGET NODE                                            
     * 
     *            5
     *          /   \
     *         0     6
     *        / \   /  \
     *       1   2  N   7
     *        \   
     *         3   
     * 
     *      
     *      
     *    SOURCE
     *         0     
     *        / \   
     *       1   2  
     *        \   
     *         3   
     * 
     */

     pub(crate) fn build_from_none(order: &mut core::slice::Iter<Option<usize>>) -> Option<Box<Node<usize>>>{ 

        let mut  c = order.next();
        
        if c.is_none(){ return None};
        let n = c.unwrap();

        if n.is_none(){ return None}

        let new = Node{
            data: n.unwrap(),
            left : build_from_none(order),
            right : build_from_none(order),
        };

        return Some(Box::new(new))
    }


    fn  build_target_node() -> Option<Box<Node<usize>>>{

        let mut  vals = [
            Some(5usize),Some(0),Some(1),None,Some(3),None, None, Some(2), None, None, Some(6),None, Some(7), None, None
        ];

        build_from_none(&mut vals.iter())

    }


    fn build_source_node() -> Option<Box<Node<usize>>>{

        let mut vals = [Some(0),Some(1),None,Some(3),None, None, Some(2), None, None];

        build_from_none(&mut vals.iter())
    }

    #[test]
    fn test_subtree_inorder(){

        let source = build_source_node();
        let target = build_target_node();
        let mut s = String::new();
        assert_eq!("1302", BT::subtree_inorder(&source, &mut s));
        let mut s = String::new();
        assert_eq!("1302567", BT::subtree_inorder(&target, &mut s));
    }

    #[test]
    fn test_subtree_preorder(){
        let source = build_source_node();
        let target = build_target_node();
        let mut s = String::new();
        assert_eq!("0132", BT::subtree_preorder(&source, &mut s));
        let mut s = String::new();
        assert_eq!("5013267", BT::subtree_preorder(&target, &mut s));
        
    }

    #[test]
    fn test_is_subtree(){

        let source = build_source_node();
        let target = build_target_node();
        let  bt = BT{root:target};
        assert!(bt.is_subtree(&source));

    }

    #[test]
    fn test_is_subtree_recr(){

        let source = build_source_node();
        let target = build_target_node();
        let  bt = BT{root:target};
        assert!(bt.is_subtree_recr(&source));

    }

    
}