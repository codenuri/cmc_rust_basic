fn ptype1(v : i32) {
	println!("{}", std::any::type_name_of_val(&v));
}

fn ptype2<T>(v : T) {
	println!("{}", std::any::type_name_of_val(&v));
}

fn main() {
	
	ptype1(3);		// ok
//	ptype1(3.4);	// error

	ptype2(3);		// ok
	ptype2(3.4);	// ok
	ptype2("A");	// ok
}
