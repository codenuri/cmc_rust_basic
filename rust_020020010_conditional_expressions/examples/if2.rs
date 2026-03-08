fn main()
{
	let n = 5;
	let b = true;

	if  n < 0  {}	// ok
//	if (n < 0) {} 	// warning: unnecessary parentheses 
				  	//     	  around `if` condition 

	if n != 0  {} 	// ok
	if b	   {}	// ok
//	if n	   {} 	// error[E0308]: mismatched types, 
					// 				 expected `bool`, 
					// 				 found integer

//	if n < 0
//		println!("negative"); // error: expected `{`
}