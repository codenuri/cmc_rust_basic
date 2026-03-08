fn main() {

    let s1 = String::from("I like Apple. Apple is delicious.");

    // replace: 새 String 반환
    let s2 = s1.replace("Apple", "Orange");
    println!("original: {:?}", s1);
    println!("replaced: {:?}", s2);

    // replace_range: 자신 변경
    let mut s3 = String::from("Hello, world!");

    s3.replace_range(7..12, "Rust");
    println!("replace_range: {:?}", s3);
}