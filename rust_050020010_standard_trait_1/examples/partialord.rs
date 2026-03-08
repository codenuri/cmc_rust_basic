use std::cmp::Ordering;

struct Point { 
	x: i32, 
	y: i32,
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.x, self.y).partial_cmp(&(other.x, other.y))
    }
}


fn main() {

	let pt1 = Point{x:1, y:1};
	let pt2 = Point{x:1, y:1};

	println!("{}", pt1 < pt2);
	println!("{}", pt1 > pt2);
	println!("{}", pt1 <= pt2);
	println!("{}", pt1 >= pt2);	
}