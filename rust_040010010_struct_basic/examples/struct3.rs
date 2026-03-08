struct Point { 
	x : i32, 
	y : i32, 
}

fn main() {
	// #1. Direct Initialization
	let pt : Point = Point{x:1, y:1};
	let pt         = Point{x:1, y:1};
	let pt         = Point{y:1, x:1};

	// #2. mutable/immutable
	let     pt = Point{x:1, y:1};
	let mut pt = Point{x:1, y:1};

	// #3. Field Init Shorthand
	let x = 1;
	let y = 1;

	let pt = Point{x:x, y:y};
	let pt = Point{x,   y};
	let pt = Point{x:x, y};

	// #4. struct update syntax
	let pt = Point{x:1, y:1};
	let p1 = Point{x:3, ..pt};	// y = pt.y
	let p2 = Point{y:3, ..pt};	// x = pt.x	
	let p3 = Point{..pt};	    // x = pt.x, y = pt.y	
}
