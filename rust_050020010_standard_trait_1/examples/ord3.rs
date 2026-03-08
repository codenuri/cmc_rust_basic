use std::cmp::Ordering;

struct Point { x: i32, y: i32 }

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Point {}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

fn main() {
	
    let mut v = vec![
        Point { x: 2, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 1, y: 2 },
    ];

    v.sort(); // ok

    for p in &v {
        println!("({}, {})", p.x, p.y);
    }
}