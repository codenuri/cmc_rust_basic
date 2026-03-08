struct Packet { 
	cmd : i32, 
	msg : Option<String>, 
}

fn main() {

	let pkt = Packet{cmd:1, msg:Some(String::from("Hello"))};

	match pkt {
		
		Packet{cmd:1, msg:Some(ref s)} => println!{"{s}"},
		_ => {},
	}

	if let Packet{cmd:1, msg:Some(ref s)} = pkt {
		println!{"{s}"}
	}
}
