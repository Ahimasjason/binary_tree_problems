
use super::*;


fn is_skewed<T>(node: Option<&Box<Node<T>>>) -> bool 
{
    if node.is_none(){
        return true
    }

    if node.unwrap().left.is_none() || node.unwrap().right.is_none(){ 
        return true
    }


    if node.unwrap().left.is_some() && node.unwrap().right.is_some(){
        return false
    }

    if node.unwrap().left.is_some(){
        is_skewed(node.unwrap().left.as_ref())
    } else {
        is_skewed(node.unwrap().right.as_ref())
    }
}





#[cfg(test)]
mod tests {

    use super::*;


    fn build() -> Option<Box<Node<usize>>>{
         let mut vals  = [
            Some(1),Some(2), Some(4),None,None,Some(6),Some(8),None,None,Some(7),None,None,Some(3),None,Some(5),None,None
        ];

        let tree = crate::sub_tree_of_another::tests::build_from_none(&mut vals.iter());

        tree
    }

    #[test]
    fn test_not_skewed(){
        let node = build();
        let sk = is_skewed(node.as_ref());
        assert!(!sk);
    }

    #[test]
    fn test_skewed(){
        let mut node = Some(Box::new(Node{data: 1,left: None,right: None}));

        node.as_mut().unwrap().left = Some(Box::new(Node{data: 2,left: None,right: None}));
        node.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(Node{data: 3,left: None,right: None}));
        let sk = is_skewed(node.as_ref());
        assert!(sk);
    }
}