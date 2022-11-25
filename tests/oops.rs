use md4rust::ootb::*;

#[test]
fn oops() {
    assert_eq!(
        github_md("~~Hello World~~\n"),
        "<p><del>Hello World</del></p>\n"
    );
    assert_eq!(
        commonmark("~~Hello World~~\n"),
        "<p>~~Hello World~~</p>\n"
    );
    let result = ootb_md(
        "~~Hello World~~\n",
        &MdParserFlags::new().strike_through(),
    );
    assert_eq!(result, "<p><del>Hello World</del></p>\n");
}
