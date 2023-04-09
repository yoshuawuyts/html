/// The HTML `<var>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/var)
#[doc(alias = "var")]
#[non_exhaustive]
pub struct Variable {
    sys: html_sys::text::Variable,
    _children: Vec<T>,
}
impl crate::categories::FlowContent for Variable {}
impl crate::categories::PhrasingContent for Variable {}
impl crate::categories::PalpableContent for Variable {}
impl std::convert::Into<html_sys::text::Variable> for Variable {
    fn into(self) -> html_sys::text::Variable {
        self.sys
    }
}
impl From<html_sys::text::Variable> for Variable {
    fn from(sys: html_sys::text::Variable) -> Self {
        Self { sys, _children: vec![] }
    }
}
