use std::fmt;
use std::fmt::Show;

struct SingleQuotedStr<'a> {
    s: &'a str
}

impl<'b> SingleQuotedStr<'b> {
    fn new<'a>(i: &'a str) -> SingleQuotedStr<'a> {
        SingleQuotedStr { s: i }
    }
}

impl<'a> Show for SingleQuotedStr<'a> {
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

impl<'a> Deref<str> for SingleQuotedStr<'a> {
    fn deref<'b>(&'b self) -> &'b str {
        self.s
    }
}

#[test]
fn it_works() {
    assert_eq!(format!("{}", SingleQuotedStr::new("'")).as_slice(), "''\\'''");
}
