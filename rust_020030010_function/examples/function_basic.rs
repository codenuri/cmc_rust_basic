fn add( a:i32, b:i32 ) -> i32 {

//	return a + b; 	// C Style
	a + b			// 권장
}

fn f1()       {}
fn f2() -> () {}


fn main() {

	let ret = add_sub(3, 1);

	println!("{}, {}", ret.0, ret.1);
}

fn add_sub(a:i32, b:i32) -> (i32, i32) {
	(a + b, a - b)
}