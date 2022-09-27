pub mod orwritekey;
pub mod parts;

fn trim_newline(s: &str) -> &str {
    let patrn = |c: char| c == '\r' || c == '\n';
    s.trim_end_matches(patrn)
}

#[test]
fn test_newline_trim() {
    assert_eq!(trim_newline("!\r\n"), "!");
    assert_eq!(trim_newline("!\n"), "!");
    assert_eq!(trim_newline("!\r"), "!");
}
