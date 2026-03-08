trait Printable {
    fn print(&self);
}

trait Labeled : Printable {
    fn label(&self) -> &str;
}

struct Report {
    title: String,
}

impl Printable for Report {
    fn print(&self) {
        println!("Printing report: {}", self.title);
    }
}

impl Labeled for Report {
    fn label(&self) -> &str {
        &self.title
    }
}

fn main() 
{
    let r = Report { title: String::from("Monthly Report") };

    println!("Label: {}", r.label());
}


