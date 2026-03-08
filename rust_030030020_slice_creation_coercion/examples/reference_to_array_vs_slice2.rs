fn sum1(r : &[i32;5]) -> i32 {
	let mut sum = 0;
	for e in r { sum += e; }
	sum
}
fn sum2(s : &[i32]) -> i32  {
	let mut sum = 0;
	for e in s { sum += e; }
	sum
}
fn main() {
	let a = [0, 1, 2, 3, 4];
	let v = vec![0, 1, 2, 3, 4];

	println!("{}", sum1(&a));			// ok
//	println!("{}", sum1(&a[1..4])); 	// error
//	println!("{}", sum1(&v[1..4])); 	// error
//	println!("{}", sum1(&v); 			// error

	println!("{}", sum2(a.as_slice())); // ok
	println!("{}", sum2(&a[1..4]));   	// ok
	println!("{}", sum2(&v[1..4])); 	// ok
	println!("{}", sum2(v.as_slice())); // ok
	println!("{}", sum2(&a)); 			// ok
	println!("{}", sum2(&v)); 			// ok
}
