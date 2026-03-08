fn f1(s : String) {
}

fn f2(r : &String) {
}

fn f3(r : &mut String) {
}

fn main() {
	let mut a = String::from("AAA");
	let mut b = String::from("AAA");
	let mut c = String::from("AAA");
	let mut d = String::from("AAA");

	f1(a);
	f1(b.clone());
	f2(&c);
	f3(&mut d);
}