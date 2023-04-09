/// The HTML `<sup>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sup)
#[doc(alias = "sup")]
#[non_exhaustive]
pub struct SuperScript {
    sys: html_sys::text::SuperScript,
    _children: Vec<T>,
}
impl crate::categories::FlowContent for SuperScript {}
impl crate::categories::PhrasingContent for SuperScript {}
impl crate::categories::PalpableContent for SuperScript {}
impl std::convert::Into<html_sys::text::SuperScript> for SuperScript {
    fn into(self) -> html_sys::text::SuperScript {
        self.sys
    }
}
impl From<html_sys::text::SuperScript> for SuperScript {
    fn from(sys: html_sys::text::SuperScript) -> Self {
        Self { sys, _children: vec![] }
    }
}
