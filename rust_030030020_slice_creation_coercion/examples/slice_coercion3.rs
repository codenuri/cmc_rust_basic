fn sum(s : &[i32]) -> i32  {
	let mut sum = 0;
	for e in s { sum += e; }
	sum
}
fn main() {
	let a = [0, 1, 2, 3, 4];
	let v = vec![0, 1, 2, 3, 4];

	println!("{}", sum(a.as_slice())); 	// ok
	println!("{}", sum(&a[1..4]));   	// ok
	println!("{}", sum(&v[1..4])); 		// ok
	println!("{}", sum(v.as_slice())); 	// ok
	println!("{}", sum(&a)); 			// ok
	println!("{}", sum(&v)); 			// ok
	
//	println!("{}", sum(a)); 	// error
//	println!("{}", sum(v)); 	// error
}
