fn main(){
	
	let v = vec![0, 1, 2, 3, 4];

	let r : &Vec<i32> = &v; 	
	let s : &[i32]    = &v;

	println!("{:p}", r);
	println!("{:p}", s);
}
