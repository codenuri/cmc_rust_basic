// n 번째 char 의 byte index 얻기
fn byte_index_of_nth_char(s: &String, n: usize) -> Option<usize> {
    s.char_indices().nth(n).map(|(i, _)| i) 
}

fn replace_nth_char(s: &mut String, n: usize, new_ch: char) -> Result<(), ()> {
    
    let start = byte_index_of_nth_char(&s, n);
	let Some(start) = start else { return Err(()); };

    let end = byte_index_of_nth_char(&s, n + 1);
	let Some(end) = end else { return Err(()); };

    s.replace_range(start..end, &new_ch.to_string());
    Ok(())
}

fn main() {
    let mut s = String::from("A가BC");

    replace_nth_char(&mut s, 1, 'D').unwrap(); // '가' -> 'D'

    println!("{s}"); // ADBC
}
