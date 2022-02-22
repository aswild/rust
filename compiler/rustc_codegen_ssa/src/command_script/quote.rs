use std::fmt;

/// Characters that must be quoted according to
/// https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html
const CHARS_NEED_QUOTE: &[char] = &[
    // The application shall quote the following characters if they are to represent themselves
    '|', '&', ';', '<', '>', '(', ')', '$', '`', '\\', '"', '\'', ' ', '\t', '\n',
    // The following may need to be quoted under certain circumstances (note: excluded '=' and '%'
    // from this list)
    '*', '?', '[', '#', '~',
    // not mentioned in the spec, but include these for consistency too
    '\r', ']', '{', '}', '!',
];

pub(super) struct ShellQuote<'a>(pub &'a str);

impl fmt::Display for ShellQuote<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // special case - empty args should be quoted
        if self.0.is_empty() {
            return f.write_str("''");
        }

        // if we don't contain any characters that need to be quoted, then just print directly
        if !self.0.contains(CHARS_NEED_QUOTE) {
            return f.write_str(self.0);
        }

        // At least one character that needs to be quoted, use single quotes, and stop quoting
        // embed single quotes we have to stop quoting e.g. "don't panic" -> 'don'\''t panic'
        f.write_str("'")?;
        let mut s = self.0;
        loop {
            match s.find('\'') {
                Some(i) => {
                    // write everything before the quote, then close the quote, add an escaped
                    // quote, and re-open the quote
                    write!(f, "{}'\\''", &s[..i])?;
                    // continue with everything after the rest of the string. If `i` was the last
                    // character in the string that's OK, the s slice will just be empty
                    s = &s[(i + 1)..];
                }
                None => {
                    // no single quote found, we can just write everything
                    f.write_str(s)?;
                    break;
                }
            }
        }
        f.write_str("'")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shellquote() {
        let test_cases = [
            ("hello_world", "hello_world"),
            ("hello world", "'hello world'"),
            ("$foobar", "'$foobar'"),
            ("don't stop me now", r"'don'\''t stop me now'"),
            ("'beginning_quote", r"''\''beginning_quote'"),
            ("end quote'", r"'end quote'\'''"),
            ("no/quotes/for/me_here+this-is:all::fine", "no/quotes/for/me_here+this-is:all::fine"),
            ("", "''"),
        ];
        for (input, expected) in test_cases {
            let actual = format!("{}", ShellQuote(input));
            assert_eq!(expected, actual);
        }
    }
}
