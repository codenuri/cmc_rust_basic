fn main() {

	let text = String::from("alpha beta\ngamma  delta\nrust");

	// 조사
    println!("contains 'beta'? {}", 	text.contains("beta"));
    println!("starts_with 'alpha'? {}", text.starts_with("alpha"));
    println!("ends_with 'rust'? {}", 	text.ends_with("rust"));

	// 검색 : Option<usize> 반환
    println!("find 'gamma' = {:?}", text.find("gamma"));
    println!("rfind 'a' = {:?}", 	text.rfind('a'));

	// 줄단위 분해
    println!("lines():");
    for line in text.lines() {
        println!("  line = {:?}", line);
    }

	// 공백 기준 분해
    println!("split_whitespace():");
    for token in text.split_whitespace() {
        println!("  token = {:?}", token);
    }

	// 기준 전달
    println!("split('\\n'):");
    for part in text.split('\n') {
        println!("  part = {:?}", part);
    }
}