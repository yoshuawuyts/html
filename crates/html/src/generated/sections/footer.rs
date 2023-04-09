/// The HTML `<footer>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/footer)
#[doc(alias = "footer")]
#[non_exhaustive]
pub struct Footer {
    sys: html_sys::sections::Footer,
}
impl crate::categories::FlowContent for Footer {}
impl crate::categories::PalpableContent for Footer {}
impl std::convert::Into<html_sys::sections::Footer> for Footer {
    fn into(self) -> html_sys::sections::Footer {
        self.sys
    }
}
impl From<html_sys::sections::Footer> for Footer {
    fn from(sys: html_sys::sections::Footer) -> Self {
        Self { sys }
    }
}
