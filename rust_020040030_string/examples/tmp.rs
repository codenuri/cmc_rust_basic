// 01_create & inspect
/*
fn main() {
    let s1 = "hello".to_string();
    let s2 = String::from("world");

    let mut s3 = String::new();
    s3.push_str("rust");

    let s4 = String::with_capacity(32);
    let s5 = "ab".repeat(3);

    println!("s1 = {:?}", s1);
    println!("s2 = {:?}", s2);
    println!("s3 = {:?}", s3);
    println!("s4 = {:?} (len={}, cap={})", s4, s4.len(), s4.capacity());
    println!("s5 = {:?} (len={}, cap={})", s5, s5.len(), s5.capacity());

    // as_str()는 &str 뷰를 얻는 API (자세한 &str 설명은 다음 강의에서)
    println!("s1.as_str() = {}", s1.as_str());
    println!("s3 is_empty? {}", s3.is_empty());
}
*/

// 02_ 
// examples/02_modify_push_insert_remove.rs
/*
fn main() {
    let mut s = String::from("Hello");

    // append
    s.push(' ');
    s.push_str("Rust");
    println!("after push/push_str: {:?}", s);

    // insert: 인덱스는 "바이트 경계"여야 함 (ASCII면 보통 문제 없음)
    s.insert(5, ','); // "Hello, Rust"
    println!("after insert: {:?}", s);

    // pop: 마지막 char 제거
    let last = s.pop();
    println!("after pop: {:?}, popped={:?}", s, last);

    // remove: 특정 위치의 char 제거 (바이트 경계)
    let removed = s.remove(5); // ',' 제거
    println!("after remove: {:?}, removed={:?}", s, removed);

    // truncate: 바이트 길이 기준 자르기 (문자 경계 주의)
    s.truncate(5); // "Hello"
    println!("after truncate(5): {:?}", s);

    // clear
    s.clear();
    println!("after clear: {:?}, len={}", s, s.len());
}
*/
/*
// examples/03_replace_and_range.rs
fn main() {
    let s = String::from("I like Java. Java is popular.");

    // replace: 새 String 반환
    let s2 = s.replace("Java", "Rust");
    println!("original: {:?}", s);
    println!("replaced : {:?}", s2);

    // replace_range: 제자리(in-place) 변경 (range는 바이트 인덱스)
    let mut s3 = String::from("Hello, world!");
    // "world"의 시작/끝 인덱스는 ASCII라 안전하게 직접 계산 가능
    // "Hello, " = 7 bytes, "world" = 5 bytes
    s3.replace_range(7..12, "Rust");
    println!("replace_range: {:?}", s3);
}

*/

/*
// examples/04_concat_plus_format.rs
fn main() {
    // 1) + : 왼쪽 String이 "소비"되는 형태(왜 그런지는 move에서 자세히)
    let s1 = String::from("Hello");
    let s2 = s1 + ", Rust"; // s1은 여기서 사용 불가(소비됨)
    println!("s2 = {:?}", s2);

    // 2) += : 뒤에 덧붙이기(내부적으로 append 계열)
    let mut s3 = String::from("Count: ");
    s3 += "1";
    s3 += ", 2";
    println!("s3 = {:?}", s3);

    // 3) format! : 새 String 생성, 원본을 보통 소비하지 않음
    let a = 10;
    let b = 20;
    let s4 = format!("a={}, b={}, sum={}", a, b, a + b);
    println!("s4 = {:?}", s4);
}
*/

// examples/05_utf8_no_indexing.rs
fn main() {
    let s = String::from("A가B"); // '가'는 UTF-8에서 여러 바이트

    // len()은 "문자 개수"가 아니라 "바이트 길이"
    println!("s = {:?}", s);
    println!("len(bytes) = {}", s.len());
    println!("chars().count() = {}", s.chars().count());

    // s[0] 같은 인덱싱은 금지 (컴파일 에러)
    // let x = s[0];

    println!("chars iteration:");
    for (i, c) in s.chars().enumerate() {
        println!("  char[{}] = {}", i, c);
    }

    println!("bytes iteration:");
    for (i, b) in s.bytes().enumerate() {
        println!("  byte[{}] = {}", i, b);
    }
}


*/
// examples/06_split_lines_search.rs
fn main() {
    let text = String::from("alpha beta\ngamma  delta\nrust");

    println!("contains 'beta'? {}", text.contains("beta"));
    println!("starts_with 'alpha'? {}", text.starts_with("alpha"));
    println!("ends_with 'rust'? {}", text.ends_with("rust"));

    println!("find 'gamma' = {:?}", text.find("gamma"));
    println!("rfind 'a' = {:?}", text.rfind('a'));

    println!("lines():");
    for line in text.lines() {
        println!("  line = {:?}", line);
    }

    println!("split_whitespace():");
    for token in text.split_whitespace() {
        println!("  token = {:?}", token);
    }

    println!("split('\\n'):");
    for part in text.split('\n') {
        println!("  part = {:?}", part);
    }
}








// 07_ 문자열 => 숫자 변환
/*
fn main() {
    let n = 42;
    let s = n.to_string();
    println!("n.to_string() = {:?}", s);

    let s2 = format!("hex={:X}, dec={}", n, n);
    println!("format! result = {:?}", s2);

    let input = "123";
    let value: i32 = input.parse::<i32>().expect("parse failed");
    println!("parsed value = {}", value);

    // 실패 예시(에러 처리 맛보기)
    let bad = "12x";
    let r = bad.parse::<i32>();
    println!("bad parse result = {:?}", r);
}
*/

// 08_read_line
/*
use std::io::{self, Read};

fn main() {
    println!("Type a line and press Enter:");

    // read_line 예제
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read_line failed");

    // trim()은 &str 뷰를 반환(자세한 &str은 다음 강의에서)
    println!("raw      = {:?}", line);
    println!("trimmed  = {:?}", line.trim());

    // 추가: 전체 stdin 읽기(여러 줄 입력을 다룰 때 감각용)
    println!("\nNow paste multiple lines, then Ctrl+Z(Windows) / Ctrl+D(Unix) to finish:");
    let mut all = String::new();
    io::stdin().read_to_string(&mut all).expect("read_to_string failed");
    println!("--- all input ---\n{}", all);
}
*/

/*
// 09_출력
fn main() {
    let s = String::from("Hello\nRust");

    // Display: 사람이 읽기 좋게
    println!("Display: {}", s);

    // Debug: 이스케이프/따옴표 포함, 상태 확인에 유리
    println!("Debug  : {:?}", s);
}
*/