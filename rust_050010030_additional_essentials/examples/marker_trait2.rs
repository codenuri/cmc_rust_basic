#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Clone for Point {
    fn clone(&self) -> Self {
        Point { x: self.x, y: self.y }
    }
}
impl Copy for Point {}

fn main() {
    let p1 = Point { x: 10, y: 20 };

    let p2 = p1; // copy

    println!("p1 = {p1:?}");
	println!("p2 = {p2:?}");

    let p3 = p1.clone();
    println!("p3 = {p3:?}");
}
