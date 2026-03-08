fn main()
{
	// formatting #1. 진법(radix)
	let v = 10;
	println!("{v}"); 	// 10		10진수
	println!("{v:b}"); 	// 1010		2 진수
	println!("{v:o}"); 	// 12		8 진수
	println!("{v:x}"); 	// a		16진수, 소문자
	println!("{v:X}"); 	// A		16진수, 대문자
	println!("");

	// formatting #2. 자릿수와 정렬
	println!("{v}",);	// '10'
	println!("{v:>6}");	// '    10'	6자리, right  align
	println!("{v:<6}");	// '10    '	6자리, left   align
	println!("{v:^6}");	// '  10  '	6자리, center align
	println!("{v:#>6}");// '####10'		
	println!("{v:#<6}");// '10####'
	println!("{v:#^6}");// '##10##'
	println!("{num:#>width$}", num=10, width=10);
							// '#######10'		
	println!("");


	// 실수 정밀도
	let f = 3.141592653589793;
	println!("{f}");	// 3.141592653589793
	println!("{f:.3}");	// 3.142	
}

