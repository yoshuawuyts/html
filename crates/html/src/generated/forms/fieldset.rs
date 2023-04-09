/// The HTML `<fieldset>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset)
#[doc(alias = "fieldset")]
#[non_exhaustive]
pub struct Fieldset {
    sys: html_sys::forms::Fieldset,
}
impl crate::categories::FlowContent for Fieldset {}
impl crate::categories::PalpableContent for Fieldset {}
impl std::convert::Into<html_sys::forms::Fieldset> for Fieldset {
    fn into(self) -> html_sys::forms::Fieldset {
        self.sys
    }
}
impl From<html_sys::forms::Fieldset> for Fieldset {
    fn from(sys: html_sys::forms::Fieldset) -> Self {
        Self { sys }
    }
}
