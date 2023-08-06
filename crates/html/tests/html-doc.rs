#![recursion_limit = "512"]

use indoc::indoc;
use pretty_assertions::assert_eq;

#[test]
fn html_doc() {
    let tree = html::root::Html::builder()
        .lang("en")
        .head(|head| {
            head.meta(|meta| meta.charset("utf-8"))
                .title(|title| title.text("My site"))
        })
        .body(|body| body.text("Hello, world!"))
        .build();

    assert_eq!(
        tree.to_string(),
        indoc!(
            r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <title>
                    My site
                </title>
            </head>
            <body>
                Hello, world!
            </body>
        </html>"#
        )
    )
}
