pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (i , elem) in array.into_iter().enumerate(){
        if *elem == key{
            return Some(i);
        }    
    }

    None 
}
