fn f1(n : i32) {
}

fn f2(r : &i32) {
//	*r = 10; // error
}

fn f3(r : &mut i32) {
	*r = 10; // ok
}

fn main() {
	let mut a = 10;
	let mut b = 10;
	let mut c = 10;

	f1(a);
	f1(a.clone());
	f2(&b);
	f3(&mut c);
}

