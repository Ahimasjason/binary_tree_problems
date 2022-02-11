


use super::search_tree::Node;


fn _convert_array_to_bst<T:Copy>(arr : &Vec<T>, start: isize, end:isize)-> Option<Box<Node<T>>> {

    if start > end {return None}


    let mid = (start + end) / 2;
    if mid < 0 { return None}
    let node = Node{data: arr[mid as usize], 
                            left: _convert_array_to_bst(arr, start, mid -1), 
                            right: _convert_array_to_bst(arr, mid+1, end)
                            
                         };

    Some(Box::new(node))
}

fn convert_array_to_bst<T:Ord+Copy>(mut arr : Vec<T>) ->  Option<Box<Node<T>>>
{

    arr.sort();
    let len = arr.len()-1;
    _convert_array_to_bst(&arr, 0, len as isize)
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_build_sorted_bst(){

        let arr = vec![ 2 , 9 , 6 , 5 , 7 , 1 , 8 , 4 ];
        let node = convert_array_to_bst(arr);   
        
        assert_eq!(node.as_ref().unwrap().data,5 );
    }
}