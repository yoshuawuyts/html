/// The HTML `<form>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form)
#[doc(alias = "form")]
#[non_exhaustive]
pub struct Form {
    sys: html_sys::forms::Form,
}
impl crate::categories::FlowContent for Form {}
impl crate::categories::PalpableContent for Form {}
impl std::convert::Into<html_sys::forms::Form> for Form {
    fn into(self) -> html_sys::forms::Form {
        self.sys
    }
}
impl From<html_sys::forms::Form> for Form {
    fn from(sys: html_sys::forms::Form) -> Self {
        Self { sys }
    }
}
