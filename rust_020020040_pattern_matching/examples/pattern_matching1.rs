fn main()
{
	let expr = 3;

	match expr
	{
		13        => println!("13"),
		1 | 3 | 5 => println!("1 | 3 | 5"),
		2..=5     => println!("2..=5"),
		_		  => println!("_"),
	}
}

