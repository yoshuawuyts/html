use html_sys::forms::Button;

#[test]
fn smoke() {
    let mut button = Button::default();
    let s = button.to_string();
    assert_eq!(s, "<button></button>");

    button.inert = Some(true);
    let s = button.to_string();
    assert_eq!(s, "<button inert></button>");
}
