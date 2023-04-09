/// The HTML `<label>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/label)
#[doc(alias = "label")]
#[non_exhaustive]
pub struct Label {
    sys: html_sys::forms::Label,
}
impl crate::categories::FlowContent for Label {}
impl crate::categories::PhrasingContent for Label {}
impl crate::categories::InteractiveContent for Label {}
impl crate::categories::PalpableContent for Label {}
impl std::convert::Into<html_sys::forms::Label> for Label {
    fn into(self) -> html_sys::forms::Label {
        self.sys
    }
}
impl From<html_sys::forms::Label> for Label {
    fn from(sys: html_sys::forms::Label) -> Self {
        Self { sys }
    }
}
