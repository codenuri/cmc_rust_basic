fn main() {

	let n : i32     = 10;
	let a : [i32;3] = [1,2,3];

	let b1 : Box<i32>     = Box::new(10);
	let b2 : Box<[i32;3]> = Box::new([1,2,3]);

	let x : i32 = *b1;
	let y : i32 = b1.pow(2); // == (*b1).pow(2)
	let z : i32 = b2[0];	 // == (*b2)[0]

	println!("{x}");
	println!("{y}");
	println!("{z}");
}
