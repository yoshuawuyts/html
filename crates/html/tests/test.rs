use html::forms::Button;
use html::text_content::OrderedList;

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
        r#"<button disabled name="testbutton">hello world</button>"#
    );
}

#[test]
fn builder() {
    let tree = OrderedList::builder()
        .list_item(|li| li.text("hello").class("pigeon").end())
        .list_item(|li| li.text("world").class("pigeon").end())
        .build();
    assert_eq!(
        tree.to_string(),
        r#"<ol><li class="pigeon">hello</li><li class="pigeon">world</li></ol>"#
    );
}

#[test]
fn looper() {
    let mut ol = OrderedList::builder();
    for name in ["hello", "world"] {
        ol.list_item(|li| li.text(name).end());
    }
    let tree = ol.build();
    assert_eq!(tree.to_string(), r#"<ol><li>hello</li><li>world</li></ol>"#);
}
