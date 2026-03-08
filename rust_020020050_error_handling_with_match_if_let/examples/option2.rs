fn index_of(arr: &[i32; 5], value: i32) -> Option<usize> {

    let mut i = 0;
    while i < arr.len() {
        if arr[i] == value {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn main() 
{
    let arr = [1, 2, 3, 4, 5];

    match index_of(&arr, 3) {
        Some(idx) => println!("found at index {}", idx),
        None      => println!("not found"),
    }
}