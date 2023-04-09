/// The HTML `<dfn>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dfn)
#[doc(alias = "dfn")]
#[non_exhaustive]
pub struct Definition {
    sys: html_sys::text::Definition,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for Definition {}
impl crate::categories::PhrasingContent for Definition {}
impl crate::categories::PalpableContent for Definition {}
impl std::convert::Into<html_sys::text::Definition> for Definition {
    fn into(self) -> html_sys::text::Definition {
        self.sys
    }
}
impl From<html_sys::text::Definition> for Definition {
    fn from(sys: html_sys::text::Definition) -> Self {
        Self { sys, _children: vec![] }
    }
}
