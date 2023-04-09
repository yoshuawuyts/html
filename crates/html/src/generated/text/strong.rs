/// The HTML `<strong>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/strong)
#[doc(alias = "strong")]
#[non_exhaustive]
pub struct Strong {
    sys: html_sys::text::Strong,
    _children: Vec<T>,
}
impl crate::categories::FlowContent for Strong {}
impl crate::categories::PhrasingContent for Strong {}
impl crate::categories::PalpableContent for Strong {}
impl std::convert::Into<html_sys::text::Strong> for Strong {
    fn into(self) -> html_sys::text::Strong {
        self.sys
    }
}
impl From<html_sys::text::Strong> for Strong {
    fn from(sys: html_sys::text::Strong) -> Self {
        Self { sys, _children: vec![] }
    }
}
