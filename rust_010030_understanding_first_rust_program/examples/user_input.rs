fn main() {

    let mut s = String::new();
    
	let sin = std::io::stdin();
	sin.read_line(&mut s).unwrap();
    
	println!("{s}");
}
