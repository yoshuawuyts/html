/// The HTML `<select>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select)
#[doc(alias = "select")]
#[non_exhaustive]
pub struct Select {
    sys: html_sys::forms::Select,
}
impl crate::categories::FlowContent for Select {}
impl crate::categories::PhrasingContent for Select {}
impl crate::categories::InteractiveContent for Select {}
impl crate::categories::PalpableContent for Select {}
impl std::convert::Into<html_sys::forms::Select> for Select {
    fn into(self) -> html_sys::forms::Select {
        self.sys
    }
}
impl From<html_sys::forms::Select> for Select {
    fn from(sys: html_sys::forms::Select) -> Self {
        Self { sys }
    }
}
