/// The HTML `<meter>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter)
#[doc(alias = "meter")]
#[non_exhaustive]
pub struct Meter {
    sys: html_sys::forms::Meter,
}
impl crate::categories::FlowContent for Meter {}
impl crate::categories::PhrasingContent for Meter {}
impl crate::categories::PalpableContent for Meter {}
impl std::convert::Into<html_sys::forms::Meter> for Meter {
    fn into(self) -> html_sys::forms::Meter {
        self.sys
    }
}
impl From<html_sys::forms::Meter> for Meter {
    fn from(sys: html_sys::forms::Meter) -> Self {
        Self { sys }
    }
}
