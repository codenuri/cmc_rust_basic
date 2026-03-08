fn main()
{
	let t = (3, 5, 7);

	match t {
		(3, 5, x) => println!("{x}"),	
		_ => {}
	}

	if let (3, 5, x) = t {
		println!("{x}");
	}

}
