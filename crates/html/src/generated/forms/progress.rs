/// The HTML `<progress>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/progress)
#[doc(alias = "progress")]
#[non_exhaustive]
pub struct Progress {
    sys: html_sys::forms::Progress,
}
impl crate::categories::FlowContent for Progress {}
impl crate::categories::PhrasingContent for Progress {}
impl crate::categories::PalpableContent for Progress {}
impl std::convert::Into<html_sys::forms::Progress> for Progress {
    fn into(self) -> html_sys::forms::Progress {
        self.sys
    }
}
impl From<html_sys::forms::Progress> for Progress {
    fn from(sys: html_sys::forms::Progress) -> Self {
        Self { sys }
    }
}
