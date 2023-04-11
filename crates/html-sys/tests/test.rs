use html_sys::forms::Button;

#[test]
fn smoke() {
    let mut button = Button::default();
    let s = button.to_string();
    assert_eq!(s, "<button></button>");

    button.inert = true;
    let s = button.to_string();
    assert_eq!(s, "<button inert></button>");

    button.name = Some("testbutton".into());
    let s = button.to_string();
    assert_eq!(s, r#"<button name="testbutton" inert></button>"#);

    button.data_map.insert("nori".into(), "cat".into());
    let s = button.to_string();
    assert_eq!(
        s,
        r#"<button name="testbutton" inert data-nori="cat"></button>"#
    );
}
