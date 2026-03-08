#[derive(Copy, Clone, Debug)]
struct Point { 
	x : i32, 
	y : i32, 
	z : i32,
}

fn main() {

	let mut pt = Point{x:1, y:2, z:3};

	// #1. 기본 모양
	match pt {
        Point { x:1, y,   z } => println!("A. 1, {y}, {z}"),
        Point { x:2, y:1, z } => println!("B. 2, 1, {z}"),
        Point { x,   y,   z } => println!("C. {x}, {y}, {z}"),	
	}

	// #2. | 사용
	match pt {
        Point { x:0, y, z } | Point {x:1, y, z } => {}, 
											// x == 0 또는 1
       	_ => {},
	}

	// #3, &, &mut
	match &pt {
        Point { x, y, z } => {}, // x, y, z 는 &i32
	}
	
	match &mut pt {
        Point { x, y, z } => {}, // x, y, z 는 &mut i32
	}	

	// #4. ref
	match pt {
        Point { x, y, ref z } => {}, // x, y 는 i32, z 는 &i32
	}		

	// #5. @
	match pt {
        Point { x: a @ 0..=10 , y, z } => {}, // a 는 x 의 값
		_ => {},
	}		

	// #6. if guard
	match pt {
        Point { x, y, z } if x == y => {},
		_ => {},
	}	
}
