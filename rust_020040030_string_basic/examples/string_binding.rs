fn main() {
	
	// [주의] string literal 자체는 &str 타입
    let s = "ABCD"; // s &str 타입

	println!("{}", std::any::type_name_of_val(&s));

	// #1. String associated function
    let s = String::new();
    let s = String::from("ABCD");
    let s = String::with_capacity(32);

	// #2. value.to_string()
    let s = "ABCD".to_string(); // &str => String
    let s = 10.to_string();     // {integer} => String  
    
	// #3. &str repeat method
    let s = "AB".repeat(3);    
    println!("{s}"); // "ABABAB"

	// #4. format!() 매크로
	let a = 1;
	let b = 2;
	let s = format!("a={a}, b={b}, sum={}", a + b);
	println!("{s}"); // "a=1, b=2, sum=3"	
}
