
use super::*;



impl <T: std::fmt::Debug + Copy + Ord + std::hash::Hash + std::ops::Add<Output=T> + Default> BT <T> {

    fn subtree_sum(root: &Option<Box<Node<T>>>, min_val : T) -> T {
        if root.is_none(){ return min_val}
        let sub_tree_size =  Self::subtree_sum(&root.as_ref().unwrap().left, min_val) + 
                                    Self::subtree_sum(&root.as_ref().unwrap().right, min_val);
        sub_tree_size + root.as_ref().unwrap().data
    }


    fn check_subtree_sum(root: &Option<Box<Node<T>>>,
                        level: usize,
                        count: &mut usize,
                        x : T, 
                        min_val : T,
                        ) 
    {

    
        if root.is_none() { return}

        if level == 1 && Self::subtree_sum(root, min_val) == x{
            *count+= 1;

        } else if level > 1 {
            Self::check_subtree_sum(&root.as_ref().unwrap().left, level - 1 , count, x, min_val);
            Self::check_subtree_sum(&root.as_ref().unwrap().right, level - 1 , count, x,min_val);

        }
    }

     fn sum_equal_to(&self, k: T, min_val: T) -> usize{

        let mut count = 0;

        let height = self.height();
        
        if height < 0 {
            return 0
        }
        for i in 1 .. (height as usize + 2){
            Self::check_subtree_sum(&self.root, i,&mut count, k, min_val);
        }

        count
    }




    fn sum_post_order(root: &Option<Box<Node<T>>>) -> Vec<&Box<Node<T>>>{
        if root.is_none(){

        }

        let mut s1 = Vec::new();
        s1.push(root.as_ref().unwrap());

        let mut s2 = Vec::new();

        while !s1.is_empty(){

            let n = s1.pop().unwrap();
            if n.left.is_some(){
                s1.push(n.left.as_ref().unwrap());
            }
            if n.right.is_some(){
                s1.push(n.right.as_ref().unwrap());
            }
            s2.push(n);
        } 

        s2
    }


    fn _recursive_sum(root: &Option<Box<Node<usize>>>, eq_sum: usize, count: &mut usize) -> usize{

        if root.is_none(){
            return 0
        }
        let left_count = Self::_recursive_sum(&root.as_ref().unwrap().left, eq_sum, count);
        let right_count = Self::_recursive_sum(&root.as_ref().unwrap().right, eq_sum, count);
        let sum = root.as_ref().unwrap().data + left_count + right_count;
        

        if sum == eq_sum{
            *count += 1;
        }
        sum
    }

    fn recursive_sum(root: &Option<Box<Node<usize>>>, eq_sum: usize) -> usize{

        let mut count = 0;
        Self::_recursive_sum(root, eq_sum, &mut count);
        count 
        
    }

    fn iterative_sum(&self, eq_sum: T) -> usize{

        let mut count = 0;

        let mut stack = Self::sum_post_order(&self.root);

        while !stack.is_empty(){

            let pointer = stack.pop().unwrap();
            if pointer.left.is_none() || pointer.right.is_none(){
                if pointer.data == eq_sum{
                    count += 1;
                    continue
                }
            }

        
            let mut s1 = Vec::new();
            let mut s2 = Vec::new();
            s1.push(pointer);
            while !s1.is_empty(){
                let p = s1.pop().unwrap();

                if p.left.is_some(){
                    s1.push(p.left.as_ref().unwrap());
                }
                if p.right.is_some(){
                    s1.push(p.right.as_ref().unwrap());
                }
                s2.push(p);
            }

            let mut sum = T::default();
            // eprintln!("default sum {:?}", sum);
            while !s2.is_empty(){

                sum = sum + s2.pop().unwrap().data;
            }

            eprintln!("after sum {:?}", sum);
            if sum == eq_sum {
                count += 1;
            }


        }


        count
    }    
}


#[cfg(test)]
mod tests {

    use super::*;


    /**
     * 
     *           1
     *         /   \
     *        3     2
     *       / \   / \
     *      2   6    11
     * 
     *  [2,6,3,11,2,1]
     * */ 

    fn build_from_none(order: &mut core::slice::Iter<Option<usize>>) -> Option<Box<Node<usize>>>{ 

        let n = order.next().unwrap();

        if n.is_none(){ return None}

        let new = Node{
            data: n.unwrap(),
            left : build_from_none(order),
            right : build_from_none(order),
        };

        return Some(Box::new(new))
    }
    fn get_node() -> Option<Box<Node<usize>>>{ 

        let mut order = vec![
            Some(1),Some(3), Some(2), None, None, Some(6),None, None, Some(2),None, Some(11),None, None
        ];

        build_from_none(&mut order.iter())
    
    }


    #[test]
    fn test_sum(){ 

        let t = get_node();
        let tree = BT{root:t};
        // eprintln!("node --> {:?}", t);
        assert_eq!(2,tree.sum_equal_to(11,0));
    }

    #[test]
    fn test_post_order(){
        let t = get_node();
        let mut p = BT::sum_post_order(&t);

        let mut i = 0;
        let p_order = [2,6,3,11,2,1];
        while i < p_order.len(){
            assert_eq!(p.pop().unwrap().data ,p_order[i]);
            i+= 1;
        }
        
    }

    #[test]
    fn test_iterative_order(){
        let t = get_node();
        let tree = BT{root: t};

        assert_eq!(2, tree.iterative_sum(11));
    }


    #[test]
    fn test_recursive_sum(){
        let t = get_node();
        // let tree = BT{root: t};

        assert_eq!(2, BT::<usize>::recursive_sum(&t, 11));
    }

    
}