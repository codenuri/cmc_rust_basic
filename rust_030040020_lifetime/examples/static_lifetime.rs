fn f1() -> &'static i32 {
	static N : i32 = 10;	
	&N
}

static G:i32 = 10;
fn f2() -> &'static i32 { 
	&G 
}

fn f3() -> &'static str { 
	"hello" 
} 

fn main(){

	let r1 = f1();
	let r2 = f2();
	let r3 = f3();

}

fn f4() -> &'static i32 { 
	&10 	// error
}