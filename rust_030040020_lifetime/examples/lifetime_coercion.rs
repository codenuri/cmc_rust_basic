
fn f1<'a>(arg : &'a i32) -> &'a i32 {

	static N : i32 = 10;	
	&N		// 긴수명 => 짧은 수명 으로 변환
}

fn main(){

	let r;
	{
		let n = 0;
		r = f1(&n);
	}
	println!("{r}"); // error	
}






fn f2<'a>(arg : &'a i32) -> &'static i32 {
	arg		// 짧은 수명 => 긴 수명 
			// error
}
