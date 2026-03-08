fn main()
{
    let n = 1;

	// #1. literal pattern
    match n {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("other"),
    }

	// #2. wildcard pattern, binding pattern
    match n {
        _ => println!("anything"),   // wildcard pattern
    }
    match n {
        x => println!("bind: {x}"),  // binding pattern 
    }

	// #3. range pattern
    match n {
        0..70 	 => println!("fail"),
        70..=100 => println!("pass"),
        _        => panic!(), 
    }	

	// #4. OR pattern
    match n {
        1 | 2    => println!("1 | 2 "),
        3..5 | 9 => println!("3..5 | 9"),
        _ 		 => {}
    }	

	// #6. guard
	let n = 12;
	match n {
        0..10 			=> println!("single digit"),
        x if x % 2 == 0 => println!("even : {x}"),
        _ 				=> {}
	}

	// #7. at-pattern
	let n = 6;

    match n {
        0..10      => println!("0..10"), 
		x @ 10..20 => println!("10..20: {x}"), 
		x @ 20..30 if x % 2 == 0 => println!("even in 20..30: {x}"),
        _ 		  => {},
    }

	// #7. tuple/struct/enum destructuring pattern
	let t = (1, 2, 3);

	match t {
		(2, _, _ ) => println!("(2, _, _)"),
		(1, 3, _ ) => println!("(1, 3, _)"),
		(_, 2, x ) => println!("(_, 2, {x})"),
		_ 		   => println!("_"),
	}

	// #8. ref pattern
	let n = 3;
	match n {
		x => println!("{}", std::any::type_name_of_val(&x)),	// i32	
	}	
	match n {
		ref x => println!("{}", std::any::type_name_of_val(&x)),// &i32
	}

	// #9. & pattern
	let r = &10;

	match r {
		x => println!("{}", std::any::type_name_of_val(&x)), // &i32
	}
	match r {
		&x => println!("{}", std::any::type_name_of_val(&x)), // i32
	}

}
