type Int = i32; 	

fn main() {
	
	let v1 : i32 = 10;
	let v2 : Int = 10;

	let v3 : i32 = v2; // ok. v2, v3 는 같은 타입

	println!("{v1}");
	println!("{v2}");

	
}