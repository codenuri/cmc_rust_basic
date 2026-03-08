fn main()
{
	let arr = [(1, 10), (1, 30), (0, 0)];

	let mut i = 0;

	while let (1, x) = arr[i] {

		print!("{x}, ");
		i += 1;
	}
}
