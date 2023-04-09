/// The HTML `<body>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body)
#[doc(alias = "body")]
#[non_exhaustive]
pub struct Body<T: crate::categories::FlowContent> {
    sys: html_sys::sections::Body,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> std::convert::Into<html_sys::sections::Body>
for Body<T> {
    fn into(self) -> html_sys::sections::Body {
        self.sys
    }
}
impl<T: crate::categories::FlowContent> From<html_sys::sections::Body> for Body<T> {
    fn from(sys: html_sys::sections::Body) -> Self {
        Self { sys, _children: vec![] }
    }
}
