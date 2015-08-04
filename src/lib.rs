use std::fmt;
use std::ops::Deref;

pub struct SingleQuotedStr<'a> {
    s: &'a str
}

impl<'b> SingleQuotedStr<'b> {
    pub fn new<'a>(i: &'a str) -> SingleQuotedStr<'a> {
        SingleQuotedStr { s: i }
    }
}

impl<'a> fmt::Display for SingleQuotedStr<'a> {
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

impl<'a> Deref for SingleQuotedStr<'a> {
    type Target = str;
    fn deref<'b>(&'b self) -> &'b Self::Target {
        self.s
    }
}

pub struct Seperated<'a, D: fmt::Display, X: fmt::Display, I: Iterator<Item=X>> {
    sep: D,
    inner: &'a Fn() -> I,
}

impl<'a, D: fmt::Display, X: fmt::Display, I: Iterator<Item=X>> fmt::Display for Seperated<'a, D, X, I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ref i in (self.inner)() {
            try!(write!(f, "{}{}", i, self.sep));
        }

        Ok(())
    }
}

impl<'a, D: fmt::Display, X: fmt::Display, I: Iterator<Item=X>> Seperated<'a, D, X, I> {
    pub fn new(sep: D, inner: &'a Fn() -> I) -> Self {
        Seperated { sep: sep, inner: inner }
    }
}


#[test]
fn test_sqs() {
    assert_eq!(format!("{}", SingleQuotedStr::new("'")), "''\\'''");
    assert_eq!(format!("{}", SingleQuotedStr::new("a")), "'a'");
}

#[test]
fn test_sep() {
    let x = [1, 2, 3];
    // FIXME: figure out how to use a literal
    assert_eq!(format!("{}", Seperated::new(' ', &|| x.iter())), "1 2 3 ");
}
