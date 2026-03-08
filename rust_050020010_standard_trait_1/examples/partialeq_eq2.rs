struct Point {
	x : i32,
	y : i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

fn main() {

	let pt1 = Point{x : 1, y : 1};
	let pt2 = Point{x : 2, y : 2};

	println!("{}", pt1 == pt2); // false
	println!("{}", pt1 != pt2); // true
}
