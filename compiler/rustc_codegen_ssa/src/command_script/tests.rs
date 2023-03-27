use super::quote::ShellQuote;

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
