fn main() {

	let v = vec![1, 2, 3, 4, 5];
	
	let n1 : i32 = v.iter().sum();
	let n2       = v.iter().sum::<i32>();
	let n3       = v.iter().product::<i32>();
}
