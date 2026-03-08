fn ptype<T>(v : T) {

	println!("{}", std::any::type_name_of_val(&v));
}

pub fn main() {

	ptype(3);	
	ptype(3.4);	
}

