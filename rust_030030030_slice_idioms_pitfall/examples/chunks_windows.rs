fn main() {

    let a = [1, 2, 3, 4, 5, 6, 7];

    for s in a.chunks(3) {
        print!("{s:?}, "); // [1,2,3], [4,5,6], [7] 
    }    
	println!();
	
	//-------------------------------------
    let mut it = a.chunks_exact(3);

    while let Some(s) = it.next() {
        print!("{s:?}, ");	// [1,2,3], [4,5,6] 	
    }
    let s = it.remainder();
    println!("remainder: {s:?}"); // remainder : [7]	

	//---------------------------------------
    for s in a.windows(3) {
        print!("{s:?}, "); // [1,2,3], [2,3,4], [3,4,5] 
    }    				   // [4,5,6], [5,6,7]
	println!();
}