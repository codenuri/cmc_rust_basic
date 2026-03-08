fn main() {

	let mut v = vec![1, 2, 3, 4, 5];
	
	let mut it1 = v.iter();		
	let mut it2 = it1;

//	println!("{it1:?}"); // error
						 // value borrowed here after move
	println!("{it2:?}"); // Iter([1, 2, 3, 4, 5])
}
