fn main() {

	let mut v = vec![1, 2, 3];

//	let ret = v.get(0);	   
	let ret = v.get_mut(0);	   

	println!("{}", v.len()); // ok error
//	println!("{}", v.pop()); // error

	println!("{:?}", ret);
}

