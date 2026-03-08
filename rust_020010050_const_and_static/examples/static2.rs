fn foo() {

	static CNT1 : i32 = 0;

	CNT1 = 10;	// error. immutable

	static mut CNT2 : i32 = 0;	// ok

//	CNT2 = 10;		// error
//	let _x = CNT2;	// error 

	unsafe {
		CNT2 = 10;
		let _x = CNT2;
	}
}

fn main() {
	
	foo();	
}

