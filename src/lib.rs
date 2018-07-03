use std::fmt;
use std::ops::{Deref,DerefMut};
use std::convert::From;
use std::iter::IntoIterator;

/// Given a str, display it as a posix-shell style single quoted string with no ambiguity.
pub struct SingleQuotedStr<'a> {
    s: &'a str
}

impl<'b> From<&'b str> for SingleQuotedStr<'b> {
    fn from(i: &str) -> SingleQuotedStr {
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
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.s
    }
}

/// Given a [u8] (bytes) display it as c-style source string with escapes
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CSrcStr<'a> {
    d: &'a [u8]
}

impl<'b> From<&'b [u8]> for CSrcStr<'b> {
    fn from(d: &[u8]) -> CSrcStr {
        CSrcStr { d: d }
    }
}

impl<'a> fmt::Display for CSrcStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.d {
            match *i as char {
                'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h'|'i'|'j'|'k'|'l'|'m'|'n'|'o'|'p'|'q'|'r'|'s'|'t'|'u'|'v'|'w'|'x'|'y'|'z'|
                'A'|'B'|'C'|'D'|'E'|'F'|'G'|'H'|'I'|'J'|'K'|'L'|'M'|'N'|'O'|'P'|'Q'|'R'|'S'|'T'|'U'|'V'|'W'|'X'|'Y'|'Z'|
                '~'|'!'|'@'|'#'|'$'|'%'|'^'|'&'|'*'|'('|')'|'_'|'+'|
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|
                '`'|
                '-'| '=' | '{' | '}' | '|' | '\\' |
                '['| ']' | ':' | ';' | '\''|
                '<'| '>' | ',' | '.' | '?' | '/' |
                ' '
                => {
                    write!(f, "{}", *i as char)?;
                },
                '"' => {
                    write!(f, "\\\"")?;  
                },
                '\n' => {
                    write!(f, "\\n")?;
                }
                _ => {
                    write!(f, "\\x{:02x}", *i)?;
                }
            }
        }

        Ok(())
    }
}

impl<'a> Deref for CSrcStr<'a> {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.d
    }
}

/// Display a given iterator over display-ables using by printing each display-able 1 display-able
/// seperator.
///
/// NOTE: for now, the seperator is also emitted after the last display-able. This should be
/// expected to change.
pub struct Seperated<'a, D: fmt::Display, X: fmt::Display, I: IntoIterator<Item=X> + 'a> {
    sep: D,
    inner: &'a Fn() -> I,
}

impl<'a, D: fmt::Display, X: fmt::Display, I: IntoIterator<Item=X>> fmt::Display for Seperated<'a, D, X, I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ref i in (self.inner)() {
            try!(write!(f, "{}{}", i, self.sep));
        }

        Ok(())
    }
}

impl<'a, D: fmt::Display, X: fmt::Display, I: IntoIterator<Item=X> + 'a> Seperated<'a, D, X, I> {
    pub fn new(sep: D, inner: &'a Fn() -> I) -> Self {
        Seperated { sep: sep, inner: inner }
    }
}

/// A wrapper around anything that dereferences to a byte slice (`[u8]`) that displays it's
/// contents as a hexidicimal string.
#[derive(Clone, PartialEq, Eq)]
pub struct Hs<T>(pub T);

impl<T: AsRef<[u8]>> fmt::Display for Hs<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let v = self.0.as_ref();
        for e in v {
            try!(write!(fmt, "{:02x}", e));
        }
        Ok(())
    }
}

impl<T: AsRef<[u8]>> fmt::Debug for Hs<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self)
    }
}

impl<T> Deref for Hs<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl<T> DerefMut for Hs<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A wrapper around anything that dereferences to a byte slice (`[u8]`) that displays it's
/// contents as a string using rust-like (& c-like) escapes for non ascii characters
///
/// The entire thing is wrapped in double quotes (") and any interior double quotes are escaped as
/// '\"'
#[derive(Clone, PartialEq, Eq)]
pub struct AsciiStr<T>(pub T);

impl<T: AsRef<[u8]>> fmt::Display for AsciiStr<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let v = self.0.as_ref();
        try!(write!(fmt, "\""));
        for e in v {
            match *e {
                b'"' => try!(write!(fmt, "\\\"")),
                0x20...0x7E => try!(write!(fmt, "{}", *e as char)),
                _ => try!(write!(fmt, "\\x{:02x}", e))
            }
        }
        try!(write!(fmt, "\""));
        Ok(())
    }
}

impl<T: AsRef<[u8]>> fmt::Debug for AsciiStr<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self)
    }
}

impl<T> Deref for AsciiStr<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl<T> DerefMut for AsciiStr<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[test]
fn test_asciistr() {
    assert_eq!(format!("{}", AsciiStr(&b"hello\x88"[..])), "\"hello\\x88\"");
    assert_eq!(format!("{}", AsciiStr(&b"hello\"\x88"[..])), "\"hello\\\"\\x88\"");
}

#[test]
fn test_hs() {
    assert_eq!(format!("{}", Hs(&b"hello"[..])), "68656c6c6f");
    let b: &[u8;3] = b"ab\xEB";
    assert_eq!(format!("{}", Hs(b)), "6162eb");
}

#[test]
fn test_sqs() {
    assert_eq!(format!("{}", SingleQuotedStr::from("'")), "''\\'''");
    assert_eq!(format!("{}", SingleQuotedStr::from("a")), "'a'");
}

#[test]
fn test_sep() {
    let x = [1, 2, 3];
    // FIXME: figure out how to use a literal
    assert_eq!(format!("{}", Seperated::new(' ', &|| x.iter())), "1 2 3 ");
}


