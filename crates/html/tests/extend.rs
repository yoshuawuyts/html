#![recursion_limit = "512"]

use html::{metadata, root};
use indoc::indoc;
use pretty_assertions::assert_eq;

#[test]
fn push() {
    let tree = root::Html::builder()
        .head(|head| head.push(metadata::Meta::builder().name("example").build()))
        .build();

    assert_eq!(
        format!("{tree:?}"),
        indoc!(
            r#"
        <!DOCTYPE html><html>
            <head>
                <meta name="example">
            </head>
        </html>"#
        )
    )
}

#[test]
fn extend() {
    let tree = root::Html::builder()
        .head(|head| {
            head.extend(vec![
                metadata::Meta::builder().name("first").build(),
                metadata::Meta::builder().name("second").build(),
            ])
        })
        .build();

    assert_eq!(
        format!("{tree:?}"),
        indoc!(
            r#"
        <!DOCTYPE html><html>
            <head>
                <meta name="first">
                <meta name="second">
            </head>
        </html>"#
        )
    )
}
