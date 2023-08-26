#![recursion_limit = "512"]

use html::root::Body;
use html::text_content::PreformattedText;
use indoc::indoc;
use pretty_assertions::assert_eq;

#[test]
fn empty_pre() {
    assert_eq!(
        PreformattedText::builder().build().to_string(),
        "<pre></pre>".to_string()
    )
}

#[test]
fn test_pre() {
    assert_eq!(
        PreformattedText::builder().text("test").build().to_string(),
        "<pre>test</pre>".to_string()
    )
}

#[test]
fn regular_pre() {
    let tree = Body::builder()
        .preformatted_text(|pre| pre.text("hello"))
        .build();

    assert_eq!(
        format!("{tree:?}"),
        indoc!(
            r#"<body>
                   <pre>hello</pre>
               </body>"#
        )
    )
}
