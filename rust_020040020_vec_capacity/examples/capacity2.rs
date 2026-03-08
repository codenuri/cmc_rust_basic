fn print_metadata( v: &Vec<i32> ) {
	println!("{}, {}, {:p}", v.len(), v.capacity(), v.as_ptr());
}

fn main() {

	let mut v1 = vec![];	// len = 0, capacity = 0
				
	
	let mut v2 = vec![0;10];// len = 10, capacity = 10
	v2.reserve(1000); 		// len = 10, capacity = 1010

	let mut v3 = Vec::<i32>::with_capacity(1000);
						// len = 0, capacity = 1000

	print_metadata(&v1);	// 0,  0
	print_metadata(&v2);	// 10, 1010
	print_metadata(&v3);	// 0,  1000
}