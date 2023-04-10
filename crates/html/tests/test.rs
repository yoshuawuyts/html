use html::forms::Button;

#[test]
fn smoke() {
    let mut button = Button::default();
    let s = button.to_string();
    assert_eq!(s, "<button></button>");

    button.set_disabled(true);
    let s = button.to_string();
    assert_eq!(s, "<button disabled></button>");

    button.set_name(Some("testbutton".to_owned()));
    let s = button.to_string();
    assert_eq!(s, r#"<button disabled name="testbutton"></button>"#);

    button.children_mut().push("hello world".into());
    let s = button.to_string();
    assert_eq!(
        s,
        r#"<button disabled name="testbutton">hello world</button>"#
    );
}
