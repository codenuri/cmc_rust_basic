fn main() {

	let t = (3, String::from("A"), String::from("B"));

	let _a = t; // t 전체가 move 됨

//	let _a = t;		// error
//	let _a = t.0;	// error
//	let _a = t.1;	// error
//	let _a = t.2;	// error

	let t = (3, String::from("A"), String::from("B"));

	let _a = t.1;	// t.1 만 move, t 는 partially move

//	println!("{:?}", t);   // error
	println!("{:?}", t.0); // ok
//	println!("{:?}", t.1); // error
	println!("{:?}", t.2); // ok



	let t = (3, String::from("A"), String::from("B"));

	let (_n, ref _s1, _s2) = t;	// t.2 만 move

//	println!("{:?}", t);   // error
	println!("{:?}", t.0); // ok
	println!("{:?}", t.1); // ok
//	println!("{:?}", t.2); // error


	let t = (3, String::from("A"), String::from("B"));

	match t {
		(_n, _s1, _s2) => {},
	}
//	println!("{:?}", t);   // error
	println!("{:?}", t.0); // ok
//	println!("{:?}", t.1); // error
//	println!("{:?}", t.2); // error


	let mut t = (3, String::from("A"), String::from("B"));

	let _a = t.1; // partially move ( t.1 )
	t.1 = String::from("A"); 

	println!("{:?}", t);   // ok
	println!("{:?}", t.0); // ok
	println!("{:?}", t.1); // ok
	println!("{:?}", t.2); // ok
}
