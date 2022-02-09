

use super::*;

use std::rc::Rc;
use std::cell::RefCell;



// fn convert<T>(mut node: Option<Box<Node<T>>>)


// {


// }


#[derive(Debug)]
pub struct DllNode<T>{
    data : T,
    left : Option<Rc<RefCell<DllNode<T>>>>,
    right : Option<Rc<RefCell<DllNode<T>>>>,
}



impl<T> DllNode <T> {

    fn new(data: T) -> Option<Rc<RefCell<DllNode<T>>>> {

        Some(
            Rc::new(
                RefCell::new(
                    DllNode{data, left:None, right: None }
                )
            )
        )
    }
}

struct DLL<T>(Option<Rc<RefCell<DllNode<T>>>>, Option<Rc<RefCell<DllNode<T>>>>, Option<Rc<RefCell<DllNode<T>>>>);
// 0 is node 1 is head 2 is prev
impl <T: Copy + Default> DLL <T> {

    fn new(node: Option<Rc<RefCell<DllNode<T>>>>) -> Self {

        DLL(node, None, None)
    }


    

    fn convert(&mut self, root : Option<Rc<RefCell<DllNode<T>>>>){

        if root.is_none(){ return }
        
        let mut  r = root.as_ref().unwrap().borrow_mut();
        self.convert(r.left.take());

        if self.2.is_none(){
            self.1 = Some(root.as_ref().unwrap().clone());
        } else {
            let prev = self.2.take().unwrap();
            r.left = Some(prev.clone());
            prev.borrow_mut().right = Some(root.as_ref().unwrap().clone()); 
        }

        self.2 = Some(root.as_ref().unwrap().clone());

        self.convert(r.right.take());
    }
}





#[cfg(test)]
mod tests {
    use super::*;


    pub(crate) fn build_from_dll_none(order: &mut core::slice::Iter<Option<usize>>) -> Option<Rc<RefCell<DllNode<usize>>>>{ 

        let mut  c = order.next();
        
        if c.is_none(){ return None};
        let n = c.unwrap();

        if n.is_none(){ return None}

        let new = DllNode{
            data: n.unwrap(),
            left : build_from_dll_none(order),
            right : build_from_dll_none(order),
        };

        return Some(Rc::new(RefCell::new(new)))
    }


    fn  build_target_node() -> Option<Rc<RefCell<DllNode<usize>>>>{

        let mut  vals = [
            Some(1usize),Some(2),Some(4),None,None, Some(5), None, None, Some(3),Some(6), None, None,None
        ];

        build_from_dll_none(&mut vals.iter())

    }

    #[test]
    fn testdll(){

        let root = build_target_node();
        eprintln!("{:?}", root);
        let mut dll = DLL(root,None,None);
        // dll.convert(root);
        // eprintln!("Linked List --> {:?}", &dll.1);
        
    }
}