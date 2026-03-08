fn add(x:i32, y:i32) -> i32 { x + y }

fn foo( fp : fn(i32, i32)-> i32 ) {

	fp(1, 2);
}

fn main() {
	
	foo( add );

	foo( |x:i32, y:i32| -> i32 { x + y } );

	foo( |x, y|  { x + y } );

	foo( |x, y|  x + y );
}


