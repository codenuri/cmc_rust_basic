struct MyType;

fn main() {

	let mt = MyType{};

	println!("{}", std::mem::size_of_val(&mt)); // 0
	
	println!("{:p}", &mt); 	// 주소 출력되지만
							// 유효하지 않은 주소
}