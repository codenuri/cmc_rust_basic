trait LogSafe {
}

struct UserId(u32);
struct Password(String);

impl LogSafe for UserId {}

fn log_value<T: LogSafe>(v: &T) {
    println!("loggable value!");
}

fn main() {

	let id = UserId(7);
    log_value(&id); // OK

    let pw = Password("secret".to_string());
// 	log_value(&pw); // error Password는 LogSafe 구현 안함
}
