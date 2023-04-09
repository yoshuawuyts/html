/// The HTML `<a>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a)
#[doc(alias = "a")]
#[non_exhaustive]
pub struct Anchor {
    sys: html_sys::text::Anchor,
}
impl crate::categories::FlowContent for Anchor {}
impl crate::categories::PhrasingContent for Anchor {}
impl crate::categories::PalpableContent for Anchor {}
impl std::convert::Into<html_sys::text::Anchor> for Anchor {
    fn into(self) -> html_sys::text::Anchor {
        self.sys
    }
}
impl From<html_sys::text::Anchor> for Anchor {
    fn from(sys: html_sys::text::Anchor) -> Self {
        Self { sys }
    }
}
