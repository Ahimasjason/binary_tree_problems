

use super::*;



use std::collections::VecDeque;



fn encode_tree<T: Copy>(node: Option<&Box<Node<T>>>, bitstr:&mut VecDeque<u8>, data_vec: &mut VecDeque<T>)

{

    if node.is_none(){ 
        bitstr.push_back(0);
        return
    }

    let data = node.as_ref().unwrap().data;
    bitstr.push_back(1);
    data_vec.push_back(data);
    encode_tree(node.as_ref().unwrap().left.as_ref(), bitstr, data_vec);
    encode_tree(node.as_ref().unwrap().right.as_ref(), bitstr, data_vec);
}

fn decode_tree<T>(bitstr:&mut VecDeque<u8>, data_vec: &mut VecDeque<T>) -> Option<Box<Node<T>>>

    where  T: Copy + Ord + std::hash::Hash + std::fmt::Debug
{

    if bitstr.is_empty(){return None}
    let x = bitstr.pop_front().unwrap();
    if x == 1 {
        let data  = data_vec.pop_front().unwrap();
        let mut n = Node::new(data);
        n.as_mut().unwrap().left = decode_tree(bitstr, data_vec);
        n.as_mut().unwrap().right = decode_tree(bitstr, data_vec);
        return n
    }
    None
}
#[cfg(test)]
mod tests {

    use super::*;

    fn build() -> Option<Box<Node<usize>>>{
         let vals  = [
            Some(1),Some(4), Some(6),None,None,Some(5),None,None,Some(2),None,Some(3),None,None
        ];
        let tree = crate::sub_tree_of_another::tests::build_from_none(&mut vals.iter());
        tree
    }


    #[test]
    fn test_encoded(){

        let r = build();
        let mut bitstr = VecDeque::new();
        let mut datastr = VecDeque::new();
        encode_tree(r.as_ref(), &mut bitstr, &mut datastr);
        let decoded = decode_tree(&mut bitstr, &mut datastr);
        assert_eq!(r.as_ref(), decoded.as_ref());
        
    }
}