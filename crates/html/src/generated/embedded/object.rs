/// The HTML `<object>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/object)
#[doc(alias = "object")]
#[non_exhaustive]
pub struct Object {
    sys: html_sys::embedded::Object,
}
impl crate::categories::FlowContent for Object {}
impl crate::categories::PhrasingContent for Object {}
impl crate::categories::EmbeddedContent for Object {}
impl crate::categories::PalpableContent for Object {}
impl std::convert::Into<html_sys::embedded::Object> for Object {
    fn into(self) -> html_sys::embedded::Object {
        self.sys
    }
}
impl From<html_sys::embedded::Object> for Object {
    fn from(sys: html_sys::embedded::Object) -> Self {
        Self { sys }
    }
}
