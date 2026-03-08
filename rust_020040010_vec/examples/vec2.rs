fn main() {

	// #1. type
//	let _v : alloc::vec::Vec<i32> = Vec::new();	// error
	let _v : std::vec::Vec<i32>   = Vec::new();	// ok
	let _v : Vec<i32> 			  = Vec::new();	// ok

	// #2. new, from associated function
	let _v : Vec<i32> = Vec::<i32>::new();
	let _v : Vec<i32> = Vec::new();
	let _v  		  = Vec::<i32>::new();
	let _v            = Vec::from([1, 2, 3]);
	let _v            = [1, 2, 3].to_vec();

	// #3. vec! 매크로
	let _v : Vec<i32> = vec![];
	let _v = vec![1,2,3,4];
	let _v = vec![0; 10];

	// #4. range & iterator
	let _v            = (0..10).collect::<Vec<i32>>();
	let _v : Vec<i32> = (0..10).collect();
	let v : Vec<i32>  = (0..10).rev().step_by(2).collect();

	println!("{v:?}"); // [9, 7, 5, 3, 1]

	// #5. mut
	let     v1 = vec![1,2,3];
	let mut v2 = vec![1,2,3];

//	v1[0] = 10; // error
	v2[0] = 10; // ok

//	v1.push(4); // error
	v2.push(4); // ok
}
