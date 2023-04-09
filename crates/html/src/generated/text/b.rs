/// The HTML `<b>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/b)
#[doc(alias = "b")]
#[non_exhaustive]
pub struct Bold {
    sys: html_sys::text::Bold,
    _children: Vec<T>,
}
impl crate::categories::FlowContent for Bold {}
impl crate::categories::PhrasingContent for Bold {}
impl crate::categories::PalpableContent for Bold {}
impl std::convert::Into<html_sys::text::Bold> for Bold {
    fn into(self) -> html_sys::text::Bold {
        self.sys
    }
}
impl From<html_sys::text::Bold> for Bold {
    fn from(sys: html_sys::text::Bold) -> Self {
        Self { sys, _children: vec![] }
    }
}
