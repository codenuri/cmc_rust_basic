struct Color1 {
	red  :u8,
	green:u8,
	blue :u8,
}

struct Color2(u8, u8, u8);

fn main() {

	let c1 = Color1{red:0, green:0, blue:0};
	let c2 = Color2(0, 0, 0);
	let c3 = (0, 0, 0);

	println!("{}, {}, {}", c1.red, c1.green, c1.blue);
	println!("{}, {}, {}", c2.0, c2.1, c2.2);
	println!("{}, {}, {}", c3.0, c3.1, c3.2);
}
