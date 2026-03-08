static MAX : i32 = 128;

fn foo() {

	static COUNT : i32 = 30;

	let _ = MAX;	// ok
	let _ = COUNT;	// ok
}

fn main() {

	foo();

	let _ = MAX;	// ok
	let _ = COUNT;	// error
}

