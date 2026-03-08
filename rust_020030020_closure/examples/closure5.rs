fn calc(x:i32, y:i32, pf : fn(i32, i32)->i32 ) -> i32 {
	pf(x, y)
}

fn main() {

	let x = 3;

	let ret1 = calc(1, 2, |a, b| a + b);     // ok
//	let ret2 = calc(1, 2, |a, b| a + b + x); // error

	println!("{ret1}"); // 3

}