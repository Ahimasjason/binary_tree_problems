
#[allow(dead_code)]

use std::hash::Hash;

use super::*;

fn is_isomorphic<T>(node1: Option<&Box<Node<T>>>, node2: Option<&Box<Node<T>>>) -> bool 
    where
        T : Ord

{
        if node1.is_none() && node2.is_none() { return true}

        if node1.is_none() || node2.is_none() { return false}

        if node1.unwrap().data != node2.unwrap().data { return false}


        (is_isomorphic(node1.unwrap().left.as_ref(), node2.unwrap().left.as_ref())
         && is_isomorphic(node1.unwrap().right.as_ref(), node2.unwrap().right.as_ref())
        ) || (is_isomorphic(node1.unwrap().left.as_ref(), node2.unwrap().right.as_ref())
         && is_isomorphic(node1.unwrap().right.as_ref(), node2.unwrap().left.as_ref())
        )


}


fn is_isomorphic_recr<T>(node1: Option<&Box<Node<T>>>, node2: Option<&Box<Node<T>>>) -> bool 

    where 
        T: Hash + Eq + Copy + std::fmt::Debug
{

    //  if both are null then it is isomorphic
    if node1.is_none() && node2.is_none() { return true} 

    //  is any one is null then is not isomorphic
    if node1.is_none() || node2.is_none() { return false}



    let mut v2 = Vec::<T>::new();
    let mut map = HashMap::new();

    let mut d1 = VecDeque::new();
    let mut d2 = VecDeque::new();

    d1.push_back(node1);
    d2.push_back(node2);
    while !d1.is_empty() && !d2.is_empty()
    {
        // println!("{:?} \n {:?}", d1, d2);
        if d1.len() != d2.len() { return false }


        let mut size = d1.len();

        map.clear();
        v2.clear();

        while size > 0 {

            let tmp1 = d1.pop_front().unwrap();
            let tmp2 = d2.pop_front().unwrap();

            *map.entry(tmp1.unwrap().data).or_insert(0) += 1;
            v2.push(tmp2.unwrap().data);


            if tmp1.unwrap().left.is_some(){ d1.push_back(tmp1.unwrap().left.as_ref())}
            if tmp1.unwrap().right.is_some(){ d1.push_back(tmp1.unwrap().right.as_ref())}
            if tmp2.unwrap().left.is_some(){ d2.push_back(tmp2.unwrap().left.as_ref())}
            if tmp2.unwrap().right.is_some(){ d2.push_back(tmp2.unwrap().right.as_ref())}

            size -= 1;

        }


        for n in &v2{
            if map.contains_key(n){
                *map.get_mut(n).unwrap() -= 1;
                if map[n] == 0{
                    map.remove_entry(n);
                } else if map[n] < 0 {
                        return false;
                }

            } else {
                return false;
            }
        }


        if map.len() > 0 {
            return false
        }
    }

    true
}



#[cfg(test)]
mod tests { 

    use super::*;


    fn build1() -> Option<Box<Node<usize>>>{
         let vals  = [
            Some(1),
            Some(2), 
            Some(4),
            None,
            None,
            Some(5),
            Some(7),
            None,
            None,
            Some(8),
            None,
            None, 
            Some(3),
            Some(6),
            None,
            None,
            None,
        ];
        let tree = crate::sub_tree_of_another::tests::build_from_none(&mut vals.iter());
        tree
    }

    fn build2() -> Option<Box<Node<usize>>>{
         let vals  = [
            Some(1),
            Some(3),
            None, 
            Some(6),
            None,
            None,
            Some(2),
            Some(4),
            None,
            None,
            Some(5),
            Some(8),
            None,
            None, 
            Some(7),
            None,
            None,
        ];
        let tree = crate::sub_tree_of_another::tests::build_from_none(&mut vals.iter());
        tree
    }
    #[test]
    fn test_is_isomorphic(){ 
        let n1 = build1();
        let n2 = build2();
        assert!(is_isomorphic_recr(n1.as_ref(), n2.as_ref()));
        assert!(is_isomorphic(n1.as_ref(), n2.as_ref()));
    }
}