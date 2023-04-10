use html::{forms::Button, text_content::Paragraph};

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

    button.children_mut().push(Paragraph::default().into());
    let s = button.to_string();
    assert_eq!(s, r#"<button disabled name="testbutton"><p></p></button>"#);
}
