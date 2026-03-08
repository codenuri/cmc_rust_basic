
fn ptype<T>(v : T) {

	println!("{}", std::any::type_name_of_val(&v));
}

fn main() {
	
	ptype(3);	
	ptype(3.4);	

	ptype::<u16>(3);	
	ptype::<f32>(3.4);	
	
	ptype::<i32>(3);	
}

