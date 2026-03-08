fn main()
{
	let t = (1, 2);
	let (a, b) = t;		// tuple destructuring(구조분해)
	
	println!("{a}, {b}"); // 1, 2


	let arr = [(1,'A'), (2, 'B'), (3, 'C')];

	for _e in arr {
		// _e 는 tuple
	}

	for (i, ch) in arr {
		println!("{i}, {ch}");
	}
}
