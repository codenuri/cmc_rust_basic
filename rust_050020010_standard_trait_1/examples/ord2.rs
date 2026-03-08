fn main() {
	
	let v1 : Vec<i32> = vec![5, 4, 3, 2, 1];
	let v2 : Vec<f64> = vec![5.5, 4.4, 3.3, 2.2, 1.1];

	v1.sort();  // ok
	
	v2.sort();	// error
				// the trait `Ord` is not implemented 
				// for `f64`
}