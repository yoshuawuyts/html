/// The HTML `<code>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/code)
#[doc(alias = "code")]
#[non_exhaustive]
pub struct Code<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::Code,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent for Code<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Code<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Code<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::Code>
for Code<T> {
    fn into(self) -> html_sys::text::Code {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::Code> for Code<T> {
    fn from(sys: html_sys::text::Code) -> Self {
        Self { sys, _children: vec![] }
    }
}
