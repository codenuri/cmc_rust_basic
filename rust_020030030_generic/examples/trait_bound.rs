fn foo<T: ToString>(v : T) {

	let _s = v.to_string();
}

fn main() {

	foo(3);
	foo(3.4);
}