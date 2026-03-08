#[derive(Copy, Clone)]
struct Point  {
	x : i32,
	y : i32,
}
/*
#[derive(Copy, Clone)]
struct Point  {	
	x : i32,
	y : String,	// error
}
*/
#[derive(Copy, Clone)]
struct PointT<T>  {
	x : i32,
	y : T,
}

fn main() {
	
	let p1 = Point{x:1, y:1};
	let p2 = PointT::<i32>{x:1, y:1};
	let p3 = PointT::<String>{x:1, y:String::from("1") };

	let _p = p1;
	let _p = p2;
	let _p = p3;

	println!("{}", p1.x);
	println!("{}", p2.x); 
	println!("{}", p3.x); // error
}
