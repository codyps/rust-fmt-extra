use std::fmt;
use std::fmt::Show;

struct QuotedStr<'a> {
    s: &'a str
}

impl<'b> QuotedStr<'b> {
    fn new<'a>(i: &'a str) -> QuotedStr<'a> {
        QuotedStr { s: i }
    }
}

impl<'a> Show for QuotedStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut elems = self.s.split('\'');
        let fst = elems.next().unwrap_or("");
        try!(write!(f, "'{}", fst));
        for elem in elems {
            try!(write!(f, "'\\''{}", elem));
        }
        write!(f, "'")
    }
}

fn main() {
    println!("{}", QuotedStr::new("hi'there'yall"))
}
