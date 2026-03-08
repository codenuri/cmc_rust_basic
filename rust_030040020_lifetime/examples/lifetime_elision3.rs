// lifetime 표기를 생략할수 없는 경우

fn f1() ->&str {	// error
	"Hello"
}

fn f2(x: &i32, y: &i32) ->&i32 { // error
	x
}

fn main() {

}