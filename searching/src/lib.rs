pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut idx :Option<usize> = None;

    for (i, elem) in array.into_iter().enumerate(){
        if *elem == key{
            idx =  Some(i);
        }
    }

    idx
}