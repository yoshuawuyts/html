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
        <html>
            <head>
                <metadata charset="utf-8"></metadata>
                <title>My site</title>
            </head>
            <body>
                Hello, world!
            </body>
        </html>
    "#
        )
    )
}
