#[test]
fn html_doc() {
    let _tree = html::root::Html::builder()
        .lang("en")
        .head(|head| {
            head.meta(|meta| meta.charset("utf-8"))
                .title(|title| title.text("My site"))
        })
        .body(|body| body.text("Hello, world!"))
        .build();
}
