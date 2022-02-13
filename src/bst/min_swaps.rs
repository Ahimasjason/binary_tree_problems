
//! Minimum swaps required to convert binary tree to bst 




fn min_swaps_to_bst<T:Ord + Copy>(arr: &Vec<T>) -> usize
{
    // know the position of an element in an arr

    let mut  paris : Vec<(T,usize)> = arr
                                            .iter()
                                            .enumerate()
                                            .map(|elem|{
                                                (*elem.1, elem.0)
                                            })
                                            .collect();

    paris.sort();
    //  now elements are in right order
    let mut ans = 0;
    // to find the cycle keep the visited flag to avoid the cycle
    let mut visited = vec![false; arr.len()];
    for i in 0..arr.len() {
        if visited[i] || paris[i].1 == i 
        // if current position is already visisted or current pos
        // already in right place move to next iteration
        {
            continue;
        }
        // this index has a cycle
        let mut cycle_size = 0;
        let mut j = i;
        while !visited[j]{
            cycle_size += 1;
            visited[j] = true;
            j = paris[j].1;
        }
        if cycle_size > 0 {
            ans += cycle_size -1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_min_swaps_to_bst(){
        let arr = vec![4,2,5,1,3];
        assert_eq!(min_swaps_to_bst(&arr), 2);
    
    }
}