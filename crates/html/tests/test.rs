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
    let ol = OrderedList::builder().list_item(|li| {}).build();
    assert_eq!(ol.to_string(), r#"<ol><li></li></ol>"#);
}
