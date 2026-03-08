fn file_size( _s : &str ) -> Result<usize, std::io::Error> {

	Ok(1024)
}

fn index_of( _arr:&[i32;5], _value : i32 ) -> Option<usize> {

	None
//	Some(1)	
}

fn main() 
{
	let arr = [1,2,3,4,5];

	let ret1 = file_size("hello.txt");
	let ret2 = index_of(&arr, 8);

	println!("{ret1:?}");
	println!("{ret2:?}");
}

