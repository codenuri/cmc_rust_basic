fn main() {

	let mut i = 0;
	let mut s = 0;

	let n = loop {				
				i = i + 1;
				s = s + i;
				if i == 10 { 
					break s; 
				//  break;	
				}
			
			};
	
	println!("{n:?}");
}
