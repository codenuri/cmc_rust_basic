struct Pos {
	x : i32, 
	y : i32,
}

struct Point<T> {
	x : T, 
	y : T, 
}

fn main() {

	let p = Pos{x:0, y:0}; 		 // ok
	let p = Point::<i32>{x:0, y:0}; // ok

	let p : Point<i32>   = Point::<i32>{x:0, y:0}; // ok
	let p : Point::<i32> = Point::<i32>{x:0, y:0}; // ok

//	let p : Point<i32>   = Point<i32>{x:0, y:0};   // error

	let p : Point<i32> = Point::<_>{x:0, y:0};	 // ok
	let p : Point<i32> = Point{x:0, y:0};		 // ok

	let p : Point<_> = Point::<i32>{x:0, y:0};   // ok
//	let p : Point    = Point::<i32>{x:0, y:0};   // error
	let p            = Point::<i32>{x:0, y:0};   // ok
}
