struct Point {
	x : i32,
	y : i32,
}

impl Drop for Point {
	fn drop(&mut self) {
		println!("drop point");
	}
}

fn main() {

	let pt1 = Point{x : 1, y : 1};
	{
		let pt2 = Point{x : 1, y : 1};

		println!("end nested {{}}");
	}	// <= call pt2.drop()

	drop(pt1); // <= call pt1.drop()

	println!("end main {{}}");
}
