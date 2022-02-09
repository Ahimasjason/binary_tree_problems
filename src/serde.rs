//! Serialize and deserialze of an array


use std::str::{FromStr, Chars};

use super::*;





fn serialize_array(arr : Vec<String>) -> String {

    let mut s = String::new();

    for x in arr{
        let delim = format!("{}{}-", '0' , x.len() );
        s.push_str(&delim);
        s.push_str(&x);
    }
    s
}

fn deserialze(serialized: &[u8]) -> Vec<String>{
    let mut position = 0;
    let mut res = Vec::new();
    while position < serialized.len(){
        while position < serialized.len() && serialized[position] != b'-'{
            position += 1;
        }
        if position >= serialized.len(){
            break;
        }

        let c  = serialized[position-1] as char;
        let length = c.to_digit(10).unwrap();
        let n = String::from_utf8_lossy(&serialized[position+ 1..position + 1 + length as usize]);
        res.push(n.into_owned());
        position += length as usize;
    }
    res
}

fn serialize_tree<T>(node: Option<&Box<Node<T>>>, stream : &mut String)

    where  T : Copy + ToString 
{
    // using preorder traversal
    if node.is_none(){ 
        stream.push_str("#");
        return
    }
    let data = format!("{}{}",node.unwrap().data.to_string(),"-");
    stream.push_str(&data);

    serialize_tree(node.unwrap().left.as_ref(), stream);
    serialize_tree(node.unwrap().right.as_ref(), stream);
}

fn _deserialize_tree_helper<'a, T>(node: &mut Option<Box<Node<T>>>, stream: &mut dyn  Iterator<Item = char>  ) -> Result<(), &'a str>
   where  T: FromStr + std::fmt::Debug + std::fmt::Display + Copy + Ord + std::hash::Hash,
{
    /*
        1-2-4-##6-8-##7-##3-#5-##
    */ 
    
    let mut data = String::new();
    while let Some(curr) = stream.next() {

        if curr == '-'{break}
        if curr == '#' {return Ok(())}
        data.push(curr);        
    }
    if data.is_empty() { return Ok(())}

    let data_val = match data.parse::<T>(){
        Ok(val) => val,
        Err(_e) => return Err("cannot convert "),
    };
    

    *node = Node::new(data_val);
    
    _deserialize_tree_helper(&mut node.as_mut().unwrap().left, stream)?;
    
    _deserialize_tree_helper(&mut node.as_mut().unwrap().right, stream)?;
    
    Ok(())

}
fn deserialze_tree<'a,T>(stream: &mut str) -> Option<Box<Node<T>>>
    where  T: FromStr + std::fmt::Debug + std::fmt::Display + Copy + Ord + std::hash::Hash,
{

    let mut node = None;
    let mut ch = stream.chars();
    let r = _deserialize_tree_helper(&mut node, &mut ch);
    
    node    
}

#[cfg(test)]
mod tests {

    use super::*;

    fn build() -> Option<Box<Node<usize>>>{
         let vals  = [
            Some(1),Some(2), Some(4),None,None,Some(6),Some(8),None,None,Some(7),None,None,Some(3),None,Some(5),None,None
        ];
        let tree = crate::sub_tree_of_another::tests::build_from_none(&mut vals.iter());
        tree
    }

    #[test]
    fn test_ser_array(){
        let a = ["OpenGenus".to_owned(), "Open".to_owned()].to_vec();
        let serialized = serialize_array(a);
        let des = deserialze(serialized.as_bytes());
        assert_eq!( des, ["OpenGenus".to_owned(), "Open".to_owned()]);
    }

    #[test]
    fn test_serialize_tree(){

        let r = build();
        let mut stream = String::new();

        serialize_tree(r.as_ref(), &mut stream);
        // eprintln!("s => {:?}", stream);
        let de = deserialze_tree::<usize>(&mut stream);
        // eprintln!("After de {:?}", de);
        assert_eq!(r.as_ref(), de.as_ref());
    }
}