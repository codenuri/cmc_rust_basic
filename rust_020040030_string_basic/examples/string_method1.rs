fn main() {

    let mut s = String::from("Hello");

    // append
    s.push(' ');			// "Hello "
    s.push_str("Rust");		// "Hello Rust"
    println!("{s:?}");

    // insert 
    s.insert(5, ','); 		// "Hello, Rust"
    println!("{s:?}");		 

    // pop
    let last = s.pop();		// "Hello, Rus"
    println!("{s:?}, popped={last:?}");	// Some(t)

    // remove
    let removed = s.remove(5); // "Hello Rus"
    println!("{s:?},  removed={removed:?}");

    // truncate: 바이트 길이 기준 자르기 
    s.truncate(5); 	// "Hello"
    println!("{s:?}");

    // clear
    s.clear();
    println!("{s:?}, len={}, capacity={}", s.len(), s.capacity()); // "", 0, 20
}