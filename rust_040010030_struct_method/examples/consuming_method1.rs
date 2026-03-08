struct Point {
    x: i32,
    y: i32,
}

impl Point {

	fn to_string( &self ) -> String {
		format!("Point({}, {})", self.x, self.y)
    }

    fn into_string( self ) -> String {
        format!("Point({}, {})", self.x, self.y)
    }

}

fn main() {
    let pt = Point { x: 1, y: 2 };

//	let s1 = pt.to_string();
	let s2 = pt.into_string();

	let x = pt.x; // error
}

