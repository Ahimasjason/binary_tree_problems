


use super::*;



/// In recursive approach 
/// 1) If x is a member, then the value of LISS(x) is 1 plus Liss of all grandchildren
/// 2) If x is not a member then the value is sum of LISS of all children


fn liss_recr<T>(node: Option<&Box<Node<T>>>) -> isize {


    if node.is_none() { return 0}

    let excl = liss_recr(node.unwrap().left.as_ref()) + liss_recr(node.unwrap().right.as_ref());

    let mut incl = 1;

    if node.unwrap().left.is_some(){ 
        incl += liss_recr(node.unwrap().left.as_ref().unwrap().left.as_ref()) + liss_recr(node.unwrap().left.as_ref().unwrap().right.as_ref());
    }
    if node.unwrap().right.is_some(){ 
        incl += liss_recr(node.unwrap().right.as_ref().unwrap().right.as_ref()) + liss_recr(node.unwrap().right.as_ref().unwrap().left.as_ref());
    }

    incl.max(excl)
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
    fn test_liss(){
        let node = build();
        let liss = liss_recr(node.as_ref());
        assert_eq!(5, liss);
    }

}