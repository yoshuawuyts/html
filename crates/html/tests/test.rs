use html::forms::Button;
use html::text_content::{Division, OrderedList, PreformattedText};
use indoc::indoc;
use pretty_assertions::assert_eq;

#[test]
fn smoke() {
    let mut button = Button::default();
    let s = button.to_string();
    assert_eq!(s, "<button></button>");

    button.set_disabled(true);
    let s = button.to_string();
    assert_eq!(s, "<button disabled></button>");

    button.set_name(Some("testbutton"));
    let s = button.to_string();
    assert_eq!(s, r#"<button disabled name="testbutton"></button>"#);

    button.children_mut().push("hello world".into());
    let s = button.to_string();
    assert_eq!(
        s,
        indoc::indoc!(
            r#"
            <button disabled name="testbutton">
                hello world
            </button>"#
        )
    );
}

#[test]
fn builder() {
    let tree = OrderedList::builder()
        .list_item(|li| li.text("hello").class("pigeon"))
        .list_item(|li| li.text("world").class("pigeon"))
        .build();
    assert_eq!(
        tree.to_string(),
        indoc!(
            r#"
            <ol>
                <li class="pigeon">
                    hello
                </li>
                <li class="pigeon">
                    world
                </li>
            </ol>"#
        )
    );
}

#[test]
fn looper() {
    let mut ol = OrderedList::builder();
    for name in ["hello", "world"] {
        ol.list_item(|li| li.text(name));
    }
    let tree = ol.build();
    assert_eq!(
        tree.to_string(),
        indoc!(
            r#"
        <ol>
            <li>
                hello
            </li>
            <li>
                world
            </li>
        </ol>"#
        )
    );
}

#[test]
fn data_attrs() {
    let mut ol = OrderedList::builder();
    for name in ["hello", "world"] {
        ol.list_item(|li| li.text(name).data("nori", "cat"));
    }
    let tree = ol.build();
    assert_eq!(
        tree.to_string(),
        indoc!(
            r#"
            <ol>
                <li data-nori="cat">
                    hello
                </li>
                <li data-nori="cat">
                    world
                </li>
            </ol>"#
        )
    );
}

#[test]
fn test_autoformat() {
    assert_eq!(
        PreformattedText::builder().text("test").build().to_string(),
        "<pre>test</pre>".to_string()
    );
    assert_eq!(
        Division::builder().text("test").build().to_string(),
        "<div>\n    test\n</div>".to_string()
    );
    assert_eq!(
        Division::builder().text("test").autoformat(Some(false)).build().to_string(),
        "<div>test</div>".to_string()
    );
    assert_eq!(
        Division::builder()
            .push(Division::builder().text("test").build())
            .autoformat(Some(false))
            .build()
            .to_string(),
        "<div><div>test</div></div>".to_string()
    );
    assert_eq!(
        Division::builder()
            .push(
                Division::builder().text("test")
                    .autoformat(Some(false))
                    .build()
            )
            .build()
            .to_string(),
        "<div>\n    <div>test</div>\n</div>".to_string()
    );
    assert_eq!(
        Division::builder()
            .push(
                Division::builder().text("test")
                    .autoformat(Some(true))
                    .build()
            )
            .autoformat(Some(false))
            .build()
            .to_string(),
        "<div><div>\n    test\n</div></div>".to_string()
    );
}

