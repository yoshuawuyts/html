/// The HTML `<var>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/var)
#[doc(alias = "var")]
#[non_exhaustive]
pub struct Variable<T: crate::categories::PhrasingContent> {
    sys: html_sys::text::Variable,
    _children: Vec<T>,
}
impl<T: crate::categories::PhrasingContent> crate::categories::FlowContent
for Variable<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PhrasingContent
for Variable<T> {}
impl<T: crate::categories::PhrasingContent> crate::categories::PalpableContent
for Variable<T> {}
impl<T: crate::categories::PhrasingContent> std::convert::Into<html_sys::text::Variable>
for Variable<T> {
    fn into(self) -> html_sys::text::Variable {
        self.sys
    }
}
impl<T: crate::categories::PhrasingContent> From<html_sys::text::Variable>
for Variable<T> {
    fn from(sys: html_sys::text::Variable) -> Self {
        Self { sys, _children: vec![] }
    }
}
