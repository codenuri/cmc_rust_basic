fn foo(r : &mut Vec<i32> ) {

	r.push(4);	// Vec::<i32>::push(&mut r, 4)
				// reborrow 발생
}

fn main() { 

	let mut v = vec![1,2,3,4,5];

	foo(&mut v);	// mutable borrow 발생

	println!("{v:?}");
}