/// The HTML `<details>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details)
#[doc(alias = "details")]
#[non_exhaustive]
pub struct Details {
    sys: html_sys::interactive::Details,
}
impl crate::categories::FlowContent for Details {}
impl crate::categories::InteractiveContent for Details {}
impl crate::categories::PalpableContent for Details {}
impl std::convert::Into<html_sys::interactive::Details> for Details {
    fn into(self) -> html_sys::interactive::Details {
        self.sys
    }
}
impl From<html_sys::interactive::Details> for Details {
    fn from(sys: html_sys::interactive::Details) -> Self {
        Self { sys }
    }
}
