
fn f1<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {	
	x 
}

fn f2<'a>(x: &'a i32, y: &i32) -> &'a i32 {	
	x 
}

fn main() {

}