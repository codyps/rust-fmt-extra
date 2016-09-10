# fmt-extra

Extra formatters (impliment fmt::Display and fmt::Debug) as wrapper types.


 - AsciiStr: print a [u8]-like as a string of ascii characters with non-visible
   characters escaped with the rust & c-style "\xFF"
 - Hs: print a [u8]-like as the hex digits without any seperators: "AF13"...
 - SingleQuotedStr: print a `&str` with posix shell style single quote escaping
 - Seperated: print an `Iterator<T: Display>` with a single Display-able
   seperator between each item.
