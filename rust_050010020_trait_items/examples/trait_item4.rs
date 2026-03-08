trait Codec {

	const HEADER_SIZE:usize;


	fn encode(&self);
	fn decode(&self);
}

struct V1;
struct V2;

impl Codec for V1 {
	
	const HEADER_SIZE:usize = 4;	
	fn encode(&self) {}
	fn decode(&self) {}
}

impl Codec for V2 {

	const HEADER_SIZE:usize = 8;		
	fn encode(&self) {}
	fn decode(&self) {}
}

fn main() {
}