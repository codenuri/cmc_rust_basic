#[derive(Copy, Clone, Debug)]
struct Point3D { 
	x : i32, 
	y : i32, 
	z : i32,
}

fn main() {

	let pt = Point{x:1, y:2, z:3};

	// #1. structure destructuring
	let Point{x:a, y:b, z:c} = pt;

	println!("{a}, {b}, {c}"); // 1, 2, 3

	
	// #2. using _, ..
	let Point{x:a, y:b, z:_} = pt;
	let Point{x:a, ..}       = pt;

	// #3. 필드명 생략
	let Point{x, y, z}   = pt; 
	let Point{x, y, z:_} = pt; 
	let Point{x, ..}     = pt; 

	// #4. ref
	let Point{x, y, ref z} = pt; // x, y 는 i32, z 는 &i32
}
