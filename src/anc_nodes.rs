use super::*;



fn _anc_recursive<T: PartialEq + Copy>(root: Option<&Box<Node<T>>>, target: Option<&Box<Node<T>>>, nodes : &mut Vec<T>) ->(bool ,Vec<T>){

    if root.is_none(){ return (false, nodes.to_vec())}

    if root == target {
        return (true, nodes.to_vec())
    }

    let left = _anc_recursive(root.as_ref().unwrap().left.as_ref(), target, nodes);

    let mut right = (false, vec![]);
    if !left.0{

        right = _anc_recursive(root.as_ref().unwrap().right.as_ref(), target, nodes);
        if right.0{
            nodes.push(root.as_ref().unwrap().data);
        }
    } else {
        nodes.push(root.as_ref().unwrap().data);
    }

    
    (left.0 | right.0, nodes.to_vec())
}



pub (crate) fn ancestor_recursive<T: PartialEq + Copy>(root: Option<&Box<Node<T>>>, target: Option<&Box<Node<T>>>) ->Vec<T>{

    _anc_recursive(root ,target, &mut Vec::new()).1

}


pub (crate) fn build_parents<T>(root: Option<&Box<Node<T>>>) -> HashMap<Option<&Box<Node<T>>>, Option<&Box<Node<T>>>>
    where  
         T: std::hash::Hash + Ord,
{

    let mut hm = HashMap::new();
    let mut stack = Vec::new();
    stack.push(root);
    hm.insert(root, None);
    while !stack.is_empty() {
        let curr = stack.pop().unwrap();
        if curr.is_some(){
            if curr.unwrap().right.as_ref().is_some(){
                stack.push(curr.unwrap().right.as_ref());
                hm.insert(curr.unwrap().right.as_ref(), curr);
            }
            if curr.unwrap().left.as_ref().is_some(){
                stack.push(curr.unwrap().left.as_ref());
                hm.insert(curr.unwrap().left.as_ref(), curr);
            }
        }
    }
    hm
}

fn ancestors<T: Copy + std::hash::Hash + Ord>(target: Option<&Box<Node<T>>>, map: HashMap<Option<&Box<Node<T>>>, Option<&Box<Node<T>>>>) -> Vec<T>{


    let mut rv = Vec::new();

    let mut curr = &target;

    while let Some(v) = map.get(curr){
            if v.is_some(){
                rv.push(v.as_ref().unwrap().data);
                curr = v;
            } else { break;}
            
    }
    rv
}



fn ancestors_iter<T: Copy + std::hash::Hash + Ord>(root: Option<&Box<Node<T>>> , target: Option<&Box<Node<T>>>) -> Vec<T>{


    let hm = build_parents(root);
    let parents = ancestors(target, hm);
    parents

}





#[cfg(test)]
mod tests {

    use super::*;


    fn build() -> Option<Box<Node<usize>>>{
         let mut vals  = [
            Some(3),Some(5), Some(6),None,None,Some(2),Some(7),None,None,Some(4),None,None,Some(1),Some(0),None,None,Some(8),None,None
        ];

        let tree = crate::sub_tree_of_another::tests::build_from_none(&mut vals.iter());

        tree
    }

    #[test]
    fn test_ancestor(){
            let tree = build();
            let mut  n = tree.as_ref();
            let mut stack = Vec::new();
            stack.push(n);
            while !stack.is_empty() {
                let curr = stack.pop().unwrap();
                if curr.is_some(){ 
                    if curr.unwrap().data == 2 {
                        n = curr;
                        break
                    }
                    if curr.unwrap().right.is_some(){
                        stack.push(curr.unwrap().right.as_ref());
                    }

                    if curr.unwrap().left.is_some(){
                        stack.push(curr.unwrap().left.as_ref());
                    }
                }
            }

            let res = ancestor_recursive(tree.as_ref(), n);
            assert_eq!(res, [5,3]);

    }


    #[test]
    fn test_none_ancestor(){
            let tree = build();
            let res = ancestor_recursive(tree.as_ref(), tree.as_ref());
            assert_eq!(res, []);

    }

    #[test]
    fn test_anc_iter(){

        let tree = build();

        let mut  n = tree.as_ref();
        let mut stack = Vec::new();
        stack.push(n);
        while !stack.is_empty() {
            let curr = stack.pop().unwrap();
            if curr.is_some(){ 
                if curr.unwrap().data == 4 {
                    n = curr;
                    break
                }
                if curr.unwrap().right.is_some(){
                    stack.push(curr.unwrap().right.as_ref());
                }

                if curr.unwrap().left.is_some(){
                    stack.push(curr.unwrap().left.as_ref());
                }
            }
        }
        let parents = ancestors_iter(tree.as_ref(), n);
        assert_eq!(parents, [2,5,3]);
    }
}