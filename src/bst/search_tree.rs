



#[derive(Debug)]
pub(crate) struct Node<T: Copy> {

    pub data : T,
    pub left : Option<Box<Node<T>>>,
    pub right : Option<Box<Node<T>>>,
}


impl <T: Copy> Node <T> {

    fn new(data: T) -> Option<Box<Node<T>>> {
        Some(Box::new(Self {data, left: None, right: None}))
    }
}




#[derive(Debug)]
struct Tree<T: Copy>{
    root : Option<Box<Node<T>>>
}

impl<T> Tree <T> 
    where  
        T : std::fmt::Debug + Ord + Copy +Clone

{

    fn new() -> Self {
        Self { root: None }
    }


    // fn _search_by_key<'a>(node : &'a Option<Box<Node<T>>>, prev : &'a Option<Box<Node<T>>>, key : T) -> ( &'a Option<Box<Node<T>>>,  &'a Option<Box<Node<T>>>){

    //     if node.is_none(){ 
    //         return (&None, &None)
    //     }

    //     if node.unwrap().data == key {
    //         return (node, prev)
    //     }

    //     let data = node.unwrap().data;
    //     if key < data {
    //         // go left
    //         Self::_search_by_key(&node.as_ref().unwrap().left, node, key)
    //     } else {
    //         // go right
    //         Self::_search_by_key(&node.as_ref().unwrap().right, node, key)
    //     }
    // }

    // fn search_by_key<'a>(self, key : T) -> ( &'a Option<Box<Node<T>>>,  &'a Option<Box<Node<T>>>)
    
    // {
    //     Self::_search_by_key(&self.root, &None, key)

    // }

    fn delete(&mut self, key : T){
        if self.root.is_none() {
            return
        }

        let r = std::mem::replace(&mut self.root, None);
        self.root = self._delete(r, key);
    }


    fn _delete(&mut self, node: Option<Box<Node<T>>>, key : T) -> Option<Box<Node<T>>> 
    {
        match node {

            Some( mut n) => {

                if n.data == key {
                    // found the node
                    // if n is leaf node then just drop the box
                    if n.left.is_none() && n.right.is_none(){
                        drop(n);
                        return None
                    }else if n.left.is_some() && n.right.is_none()
                    {
                        let left = n.left.take();
                        drop(n);
                        return left;
                    }else if n.right.is_some() && n.left.is_none()
                    {
                        let right = n.right.take();
                        drop(n);
                        return right;
                    } else {

                        let mut in_succ = n.right.as_ref();
                        eprintln!("{:?}", in_succ);

                        while in_succ.is_some() && in_succ.unwrap().left.is_some() {
                            in_succ = in_succ.unwrap().left.as_ref();
                        }
                        eprintln!("{:?}", in_succ);
                        let minkey = in_succ.unwrap().data;
                        n.data = minkey;
                        let right = n.right.take();
                        n.right = self._delete(right, minkey);
                        return Some(n)
                    }
                }
                 None
            },
            None => None
        }
    }


    fn _insert(node: Option<Box<Node<T>>>, key: T) -> Option<Box<Node<T>>> {

        match node {

            Some(mut n) => {
                if key < n.data {
                    let left = Self::_insert(n.left, key);
                    n.left =left;
                    Some(n)  
                } else {
                    let right = Self::_insert(n.right, key);
                    n.right = right;
                    Some(n)
                }

            },
            None => {

                let new = Node::new(key);
                new
            }
        }
    }
    fn insert(&mut self, key : T) {

       let r = std::mem::replace(&mut self.root, None);

       self.root = Self::_insert(r, key);
    }




    fn predcessor(&self, source: &Option<Box<Node<T>>>) -> Option<&Box<Node<T>>>{
        todo!()
    }

    fn sucessor(&self, source: &Option<Box<Node<T>>>) -> Option<&Box<Node<T>>>{
        todo!()
    }

}






#[cfg(test)]
mod tests {


    use super::Tree;


    #[test]
    fn test_insert(){

        let mut tree = Tree::new();

        let vals = &[5,3,7,2];

        for val in vals {
            tree.insert(*val);
        }

        

        // eprintln!("{:?}", tree);
        tree.delete(5);
        // eprintln!("{:?}", tree);
        assert_eq!(tree.root.as_ref().unwrap().data , 7);

    }
}