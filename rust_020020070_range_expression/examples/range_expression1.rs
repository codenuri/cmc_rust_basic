fn main() {

	let r1 = std::ops::Range{start:0u16, end:10u16};
	let r2 = std::ops::Range{start:0u32, end:10u32};
	let r3 = std::ops::Range{start:0,    end:10};

	println!("{}", std::mem::size_of_val(&r1)); // 4
	println!("{}", std::mem::size_of_val(&r2)); // 8
	println!("{}", std::mem::size_of_val(&r3)); // 8

	let r4 = 0..10;
	println!("{}", std::any::type_name_of_val(&r3));
	println!("{}", std::any::type_name_of_val(&r4));

//	println!("{r3}");	// error
	println!("{r3:?}");
	println!("{r4:?}");
}
