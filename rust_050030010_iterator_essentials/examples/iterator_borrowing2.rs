fn main() {

	let mut v = vec![1, 2, 3, 4, 5];
	
	let mut it = v.iter();

				// Some(&1st_element)
//	let Some(r) = it.next() else { panic!() };
				// r 은 Reference 이므로 v 의 대여자
//	v.push(3);	// error

	let Some(&r) = it.next() else { panic!() };
	v.push(3);  // ok

	println!("{:?}", r);
}
