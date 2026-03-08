fn main()
{
	let n = 10;

	let s : String = n.to_string(); 	 // 10 => "10"
	let v : i32    = s.parse().unwrap(); // "10" => 10

	println!("{s}");
	println!("{v}");
}
