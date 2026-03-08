fn main() {

	let n : i8 = -11;

	println!("{n}, {n:08b}");	  // -11, 11110101

	println!("{}", n.count_ones());   // 6
	println!("{}", n.count_zeros());  // 2
	println!("{}", n.is_negative());  // true
	println!("{}", n.pow(2));		  // 121

	println!("{}", 3u8.pow(2));	// 9	
}
